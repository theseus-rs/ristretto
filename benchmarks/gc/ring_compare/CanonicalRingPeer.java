import java.lang.management.GarbageCollectorMXBean;
import java.lang.management.ManagementFactory;
import java.util.List;
import com.sun.management.ThreadMXBean;

/** Peer-only observer. Shared workload bytecode is copied without recompiling. */
public final class CanonicalRingPeer {
    private static volatile int sink;

    private static int invoke(String workload, int steps) {
        return switch (workload) {
            case "small" -> CanonicalRingWorkload.small(steps);
            case "medium" -> CanonicalRingWorkload.medium(steps);
            case "large" -> CanonicalRingWorkload.large(steps);
            default -> throw new IllegalArgumentException("unknown ring workload");
        };
    }

    private static String quoted(String value) {
        return "\"" + value.replace("\\", "\\\\").replace("\"", "\\\"") + "\"";
    }

    public static void main(String[] args) {
        if (args.length > 1 && args[1].startsWith("scheduled-")) { scheduled(args); return; }
        if (args.length != 7) throw new IllegalArgumentException(
            "run_id check|warmup-check|measure|allocation workload steps warmup samples expected");
        String run = args[0], mode = args[1], workload = args[2];
        boolean measured = mode.equals("measure");
        boolean allocation = mode.equals("allocation");
        boolean warmupCheck = mode.equals("warmup-check");
        int steps = Integer.parseInt(args[3]), warmup = Integer.parseInt(args[4]);
        int samples = Integer.parseInt(args[5]), expected = Integer.parseInt(args[6]);
        if ((!measured && !allocation && !warmupCheck && !mode.equals("check")) || steps < 0 || warmup < 0 || warmup > 128
                || samples < 1 || samples > 128 || (mode.equals("check") && (warmup != 0 || samples != 1))) {
            throw new IllegalArgumentException("invalid sample plan");
        }
        long[] elapsed = new long[samples];
        int[] results = new int[samples];
        List<GarbageCollectorMXBean> beans = warmupCheck ? List.of() : ManagementFactory.getGarbageCollectorMXBeans();
        long[] before = new long[beans.size()], after = new long[beans.size()];
        ThreadMXBean threads = allocation ? (ThreadMXBean) ManagementFactory.getThreadMXBean() : null;
        long thread = Thread.currentThread().getId();
        if (allocation) {
            if (!threads.isThreadAllocatedMemorySupported()) throw new IllegalStateException("allocation counter unavailable");
            if (!threads.isThreadAllocatedMemoryEnabled()) threads.setThreadAllocatedMemoryEnabled(true);
        }
        for (int i = 0; i < warmup; i++) {
            int result = invoke(workload, steps);
            if (result != expected) throw new IllegalStateException("warmup checksum mismatch");
            sink = result;
        }
        if (measured) {
            for (int i = 0; i < beans.size(); i++) before[i] = beans.get(i).getCollectionCount();
        }
        long allocatedBefore = allocation ? threads.getThreadAllocatedBytes(thread) : 0;
        for (int i = 0; i < samples; i++) {
            long start = measured ? System.nanoTime() : 0;
            int result = invoke(workload, steps);
            long end = measured ? System.nanoTime() : 0;
            if (result != expected || end < start) throw new IllegalStateException("sample check failed");
            elapsed[i] = end - start;
            results[i] = result;
            sink = result;
        }
        long allocatedAfter = allocation ? threads.getThreadAllocatedBytes(thread) : 0;
        if (allocation && (allocatedBefore < 0 || allocatedAfter <= allocatedBefore)) {
            throw new IllegalStateException("allocation evidence unavailable");
        }
        if (measured) {
            for (int i = 0; i < beans.size(); i++) after[i] = beans.get(i).getCollectionCount();
        }
        if (warmupCheck) {
            System.out.println("RISTRETTO_RING_WARMUP_CHECK {\"schema_version\":1,\"qualification\":false,"
                + "\"identity\":{\"run_id\":" + quoted(run) + "},\"scope\":\"warmup_correctness_only\","
                + "\"completed_warmups\":" + warmup + ",\"completed_checks\":" + samples
                + ",\"expected_result\":" + expected + ",\"checksum_verified\":true,\"cleanup_verified\":null,"
                + "\"timings\":null,\"allocation_counters\":null,\"execution_counters\":null,\"gc_counters\":null,\"method_profiles\":null}");
            return;
        }
        // No GC request here: HotSpot retains its normal deferred-collection policy.
        StringBuilder out = new StringBuilder("RISTRETTO_RING_PEER {");
        out.append("\"schema_version\":1,\"qualification\":false,\"diagnostic_only\":true,");
        out.append("\"run_id\":").append(quoted(run));
        out.append(",\"sample_mode\":").append(quoted(mode));
        out.append(",\"workload\":").append(quoted(workload));
        out.append(",\"steps\":").append(steps).append(",\"warmup_invocations\":").append(warmup);
        out.append(",\"scope\":\"workload_call_deferred_gc_v1\"");
        out.append(",\"collector_lifecycle\":\"shared_hotspot_heap_deferred_gc\"");
        out.append(",\"clock\":\"java_system_nano_time\",\"samples\":[");
        for (int i = 0; i < samples; i++) {
            if (i > 0) out.append(',');
            out.append("{\"result\":").append(results[i]).append(",\"elapsed_ns\":");
            if (measured) out.append(elapsed[i]); else out.append("null");
            out.append('}');
        }
        out.append("],\"actual_thread_allocated_bytes\":");
        if (allocation) out.append(allocatedAfter - allocatedBefore); else out.append("null");
        out.append(",\"gc_observation_scope\":\"whole_sample_batch_including_driver_gaps\"");
        out.append(",\"gc_collection_counts\":[");
        for (int i = 0; i < beans.size(); i++) {
            if (i > 0) out.append(',');
            out.append("{\"name\":").append(quoted(beans.get(i).getName()));
            out.append(",\"before\":");
            if (measured && before[i] >= 0) out.append(before[i]); else out.append("null");
            out.append(",\"after\":");
            if (measured && after[i] >= 0) out.append(after[i]); else out.append("null");
            out.append('}');
        }
        out.append("],\"in_invocation_gc_exposure_proven\":false,\"global_capture_complete\":false");
        out.append(",\"scheduled_latency_available\":false,\"sustainable_throughput_available\":false}");
        System.out.println(out);
    }

    private static long release(long index, long rate) {
        return index / rate * 1000000000L + index % rate * 1000000000L / rate;
    }
    private static long arrivals(long offset, long rate, long offered) {
        long next = offset + 1;
        return Math.min(offered, next / 1000000000L * rate
            + (next % 1000000000L * rate + 999999999L) / 1000000000L);
    }
    private static long await(long deadline) {
        long now;
        while ((now = System.nanoTime()) < deadline) {
            long remaining = deadline - now;
            if (remaining >= 1000000L) {
                try { Thread.sleep(remaining / 1000000L); }
                catch (InterruptedException error) { Thread.currentThread().interrupt(); throw new IllegalStateException(error); }
            } else Thread.yield();
        }
        return now;
    }
    // Same bounded upper-bound histogram as GcBench and the canonical driver.
    private static void record(long[] counts, long ns) {
        if (ns < 0) throw new IllegalStateException("clock reversed");
        long us = ns / 1000 + (ns % 1000 == 0 ? 0 : 1);
        int index;
        if (us <= 1024) index = (int)us;
        else {
            int exponent = 63 - Long.numberOfLeadingZeros(us);
            long width = 1L << (exponent - 6);
            index = 1024 + (exponent - 10) * 64 + (int)((us - (1L << exponent) + width - 1) / width);
        }
        if (index >= counts.length) throw new IllegalStateException("histogram overflow");
        counts[index]++;
    }
    private static String histogram(long[] counts, long max) {
        StringBuilder out = new StringBuilder("{\"max_ns\":" + max + ",\"buckets_us\":[");
        boolean first = true;
        for (int i=0;i<counts.length;i++) if (counts[i] != 0) {
            int relative=i-1024, exponent=10+relative/64;
            long upper=i<=1024 ? i : (1L << exponent) + (relative%64)*(1L << (exponent-6));
            if (!first) out.append(','); first=false;
            out.append('[').append(upper).append(',').append(counts[i]).append(']');
        }
        return out.append("]}").toString();
    }
    private static void scheduled(String[] args) {
        if (args.length!=9) throw new IllegalArgumentException("scheduled arguments");
        String run=args[0],mode=args[1],workload=args[2];
        boolean measured=mode.equals("scheduled-measure");
        int steps=Integer.parseInt(args[3]),warmup=Integer.parseInt(args[4]),expected=Integer.parseInt(args[8]);
        long seconds=Long.parseLong(args[5]),rate=Long.parseLong(args[6]),cap=Long.parseLong(args[7]);
        if ((!measured&&!mode.equals("scheduled-check")) || steps<0 || warmup<0 || warmup>128
            || seconds<1 || seconds>86400 || rate<1 || rate>1000000000L || cap<1 || cap>1000000000L
            || (!measured && (warmup!=0 || cap!=3))) throw new IllegalArgumentException("scheduled plan");
        long[] service=new long[4096],response=new long[4096];
        long[][] timeline=new long[128][3]; int timelineLength=0;
        List<GarbageCollectorMXBean> beans=ManagementFactory.getGarbageCollectorMXBeans();
        long[] gcBefore=new long[beans.size()],gcAfter=new long[beans.size()];
        for (int i=0;i<warmup;i++) { int value=invoke(workload,steps); if(value!=expected)throw new IllegalStateException("warmup checksum");sink=value; }
        for(int i=0;i<beans.size();i++)gcBefore[i]=beans.get(i).getCollectionCount();
        long duration=seconds*1000000000L,offered=seconds*rate;
        long start=measured?System.nanoTime():0,deadline=start+duration;
        long completed=0,onTime=0,peakBacklog=0,checksum=0,maxService=0,maxResponse=0;
        boolean capReached=false;
        while(true) {
            if(!measured&&completed==3)break;
            long begin=0,due=0;
            if(measured) {
                long now=System.nanoTime();if(now>=deadline||completed==offered)break;
                if(completed==cap){capReached=true;break;}
                due=start+release(completed,rate);begin=await(due);if(begin>=deadline)break;
                peakBacklog=Math.max(peakBacklog,Math.max(0,arrivals(begin-start,rate,offered)-completed-1));
            }
            int value=invoke(workload,steps);long finish=measured?System.nanoTime():0;
            if(value!=expected)throw new IllegalStateException("sample checksum");
            sink=value;completed++;checksum+=value;
            if(measured) {
                record(service,finish-begin);record(response,finish-due);
                maxService=Math.max(maxService,finish-begin);maxResponse=Math.max(maxResponse,finish-due);
                if(finish<=deadline)onTime++;
                long backlog=Math.max(0,arrivals(finish-start,rate,offered)-completed);
                peakBacklog=Math.max(peakBacklog,backlog);
                if(timelineLength<128&&finish-start>=duration/128*timelineLength) {
                    timeline[timelineLength][0]=finish-start;timeline[timelineLength][1]=completed;timeline[timelineLength++][2]=backlog;
                }
            }
        }
        if(measured){await(deadline);peakBacklog=Math.max(peakBacklog,offered-completed);}
        long end=measured?System.nanoTime():0;
        for(int i=0;i<beans.size();i++)gcAfter[i]=beans.get(i).getCollectionCount();
        StringBuilder out=new StringBuilder("RISTRETTO_RING_SCHEDULED_PEER {\"schema_version\":1,\"qualification\":false,\"run_id\":").append(quoted(run));
        out.append(",\"sample_mode\":").append(quoted(mode)).append(",\"scope\":\"retained_peer_heap_scheduled_v1\",\"arrival_policy\":\"fixed_index_release_logical_fifo_v1\"");
        out.append(",\"warmup_invocations\":").append(warmup).append(",\"expected_result\":").append(expected).append(",\"checksum_verified\":true,\"completed\":").append(completed).append(",\"checksum\":").append(checksum);
        out.append(",\"schedule\":");
        if(!measured)out.append("null");else {
            out.append("{\"duration_ns\":").append(duration).append(",\"start_clock_ns\":").append(start).append(",\"end_clock_ns\":").append(end).append(",\"deadline_clock_ns\":").append(deadline);
            out.append(",\"offered\":").append(offered).append(",\"completed\":").append(completed).append(",\"deadline_completed\":").append(onTime).append(",\"deadline_backlog\":").append(offered-onTime);
            out.append(",\"after_deadline_completions\":").append(completed-onTime).append(",\"unfinished\":").append(offered-completed).append(",\"peak_backlog\":").append(peakBacklog);
            out.append(",\"completion_cap_reached\":").append(capReached).append(",\"status\":").append(quoted(capReached?"completion_cap":completed<offered?"unfinished":"complete"));
            out.append(",\"service\":").append(histogram(service,maxService)).append(",\"response\":").append(histogram(response,maxResponse)).append(",\"timeline\":[");
            for(int i=0;i<timelineLength;i++){if(i>0)out.append(',');out.append('[').append(timeline[i][0]).append(',').append(timeline[i][1]).append(',').append(timeline[i][2]).append(']');}
            out.append("]}");
        }
        out.append(",\"gc_observation_scope\":\"whole_schedule_batch_including_driver_gaps\",\"gc_collection_counts\":[");
        for(int i=0;i<beans.size();i++){if(i>0)out.append(',');out.append("{\"name\":").append(quoted(beans.get(i).getName())).append(",\"before\":");if(gcBefore[i]<0)out.append("null");else out.append(gcBefore[i]);out.append(",\"after\":");if(gcAfter[i]<0)out.append("null");else out.append(gcAfter[i]);out.append('}');}
        out.append("],\"global_capture_complete\":false,\"sustainable_throughput_available\":false}");
        System.out.println(out);
    }
}
