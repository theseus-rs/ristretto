import java.lang.foreign.Arena;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.concurrent.CountDownLatch;

/** Indirect handles must retain their coordinate and receiver adapters. */
public class Test {
    public static void main(String[] args) throws InterruptedException {
        VarHandle view = MemoryLayout.sequenceLayout(4, ValueLayout.JAVA_INT_UNALIGNED)
            .varHandle(MemoryLayout.PathElement.sequenceElement());
        MemorySegment first = MemorySegment.ofArray(new byte[16]);
        MemorySegment second = MemorySegment.ofArray(new byte[16]);
        for (int round = 0; round < 3; round++) {
            view.set(first, 0L, 2L, 123 + round);
            view.set(second, 0L, 1L, 456 + round);
            if ((int) view.get(first, 0L, 2L) != 123 + round
                    || (int) view.get(second, 0L, 1L) != 456 + round
                    || (int) view.get(first, 0L, 1L) != 0
                    || (int) view.get(second, 0L, 2L) != 0) {
                throw new AssertionError("incorrect indirect VarHandle result");
            }
        }
        nativeSegments();
        pathsAndBounds();
        atomicRestrictions();
        sharedLayout();
        closeDuringAccess();
        closedBufferView();
        System.out.println("Indirect VarHandle checks passed");
    }

    static void closedBufferView() {
        VarHandle word = MethodHandles.byteBufferViewVarHandle(int[].class, ByteOrder.nativeOrder());
        ByteBuffer buffer;
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment segment = arena.allocate(ValueLayout.JAVA_INT);
            buffer = segment.asByteBuffer();
            word.set(buffer, 0, 37);
            if (segment.get(ValueLayout.JAVA_INT, 0L) != 37) throw new AssertionError("segment buffer storage");
        }
        expect(IllegalStateException.class, () -> { int value = (int) word.get(buffer, 0); });
    }

    static void closeDuringAccess() throws InterruptedException {
        Arena arena = Arena.ofShared();
        MemorySegment segment = arena.allocate(ValueLayout.JAVA_INT);
        segment.set(ValueLayout.JAVA_INT, 0L, 37);
        CountDownLatch started = new CountDownLatch(1);
        Throwable[] failure = new Throwable[1];
        Thread reader = new Thread(() -> {
            try {
                while (true) {
                    if (segment.get(ValueLayout.JAVA_INT, 0L) != 37) {
                        throw new AssertionError("read freed storage");
                    }
                    started.countDown();
                }
            } catch (IllegalStateException closed) {
                // A read after close observes the closed session.
            } catch (Throwable t) {
                failure[0] = t;
            } finally {
                started.countDown();
            }
        });
        reader.setDaemon(true);
        reader.start();
        try {
            started.await();
        } finally {
            arena.close();
            reader.join();
        }
        if (failure[0] != null) throw new AssertionError("shared segment read", failure[0]);
    }

    static void sharedLayout() throws InterruptedException {
        ByteOrder order = ByteOrder.nativeOrder() == ByteOrder.LITTLE_ENDIAN ? ByteOrder.BIG_ENDIAN : ByteOrder.LITTLE_ENDIAN;
        ValueLayout.OfInt word = ValueLayout.JAVA_INT.withOrder(order);
        MemoryLayout row = MemoryLayout.structLayout(ValueLayout.JAVA_INT.withName("tag"),
                MemoryLayout.sequenceLayout(3, word).withName("items"));
        VarHandle item = MemoryLayout.sequenceLayout(2, row).varHandle(MemoryLayout.PathElement.sequenceElement(),
                MemoryLayout.PathElement.groupElement("items"), MemoryLayout.PathElement.sequenceElement());
        VarHandle scalar = word.varHandle();
        try (Arena arena = Arena.ofShared()) {
            MemorySegment segment = arena.allocate(40, 4);
            // Base 4 + one 16-byte row + 4-byte tag + two 4-byte items.
            long offset = 32L;
            int initial = 0x01020304;
            item.set(segment, 4L, 1L, 2L, initial);
            int firstByte = segment.get(ValueLayout.JAVA_BYTE, offset) & 0xff;
            if (firstByte != (order == ByteOrder.BIG_ENDIAN ? 1 : 4) || (int) scalar.get(segment, offset) != initial) {
                throw new AssertionError("layout path or byte order");
            }
            Thread worker = new Thread(() -> {
                for (int i = 0; i < 32; i++) item.getAndAdd(segment, 4L, 1L, 2L, 1);
            });
            worker.start();
            try {
                for (int i = 0; i < 32; i++) scalar.getAndAdd(segment, offset, 1);
            } finally {
                worker.join();
            }
            if (segment.get(word, offset) != initial + 64 || segment.get(word, offset - 4) != 0
                    || segment.get(word, offset + 4) != 0) throw new AssertionError("shared layout updates");
        }
    }

    static void pathsAndBounds() {
        int[] words = {11, 12, 13, 14};
        MemorySegment heap = MemorySegment.ofArray(words);
        MemoryLayout sequence = MemoryLayout.sequenceLayout(4, ValueLayout.JAVA_INT);
        VarHandle reverse = sequence.varHandle(MemoryLayout.PathElement.sequenceElement(3, -1));
        VarHandle fixed = sequence.varHandle(MemoryLayout.PathElement.sequenceElement(2));
        if ((int) reverse.getAndSet(heap, 0L, 1L, 27) != 13 || (int) fixed.get(heap, 0L) != 27
                || !java.util.Arrays.equals(words, new int[]{11, 12, 27, 14})) throw new AssertionError("reverse or fixed path");
        MemoryLayout union = MemoryLayout.unionLayout(ValueLayout.JAVA_INT, ValueLayout.JAVA_FLOAT);
        VarHandle bits = union.varHandle(MemoryLayout.PathElement.groupElement(0));
        VarHandle floating = union.varHandle(MemoryLayout.PathElement.groupElement(1));
        bits.set(heap, 0L, Float.floatToRawIntBits(-1.25f));
        if ((float) floating.get(heap, 0L) != -1.25f) throw new AssertionError("union aliases");

        expect(IllegalArgumentException.class, () -> ValueLayout.ADDRESS.varHandle(MemoryLayout.PathElement.dereferenceElement()));
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment slot = arena.allocate(ValueLayout.ADDRESS);
            MemorySegment left = arena.allocate(ValueLayout.JAVA_INT), right = arena.allocate(ValueLayout.JAVA_INT);
            VarHandle through = ValueLayout.ADDRESS.withTargetLayout(ValueLayout.JAVA_INT)
                    .varHandle(MemoryLayout.PathElement.dereferenceElement());
            slot.set(ValueLayout.ADDRESS, 0L, left);
            through.set(slot, 0L, 17);
            slot.set(ValueLayout.ADDRESS, 0L, right);
            through.set(slot, 0L, 31);
            if (left.get(ValueLayout.JAVA_INT, 0L) != 17 || right.get(ValueLayout.JAVA_INT, 0L) != 31) {
                throw new AssertionError("dereferenced stale address");
            }
        }
    }

    static void atomicRestrictions() throws InterruptedException {
        VarHandle word = ValueLayout.JAVA_INT.varHandle();
        VarHandle indexed = ValueLayout.JAVA_INT.arrayElementVarHandle();
        MemorySegment segment;
        try (Arena arena = Arena.ofConfined()) {
            segment = arena.allocate(ValueLayout.JAVA_INT);
            segment.set(ValueLayout.JAVA_INT, 0L, 19);
            expect(IllegalArgumentException.class, () -> word.compareAndSet(segment.asReadOnly(), 0L, 19, 27));
            expect(IndexOutOfBoundsException.class, () -> word.getAndSet(segment.asSlice(0, 0), 0L, 27));
            expect(ArithmeticException.class, () -> indexed.getAndSet(segment, 0L, Long.MAX_VALUE, 27));
            Throwable[] failure = new Throwable[1];
            Thread worker = new Thread(() -> {
                try {
                    expect(WrongThreadException.class, () -> word.getAndSet(segment, 0L, 27));
                } catch (Throwable t) { failure[0] = t; }
            });
            worker.start();
            worker.join();
            if (failure[0] != null) throw new AssertionError("confined atomic update", failure[0]);
            if (segment.get(ValueLayout.JAVA_INT, 0L) != 19) throw new AssertionError("rejected update changed storage");
        }
        expect(IllegalStateException.class, () -> word.getAndSet(segment, 0L, 27));
    }

    static void nativeSegments() throws InterruptedException {
        VarHandle element = MemoryLayout.sequenceLayout(3, ValueLayout.JAVA_INT)
                .varHandle(MemoryLayout.PathElement.sequenceElement());
        VarHandle strided = ValueLayout.JAVA_INT.arrayElementVarHandle();
        MemorySegment segment;
        try (Arena arena = Arena.ofConfined()) {
            segment = arena.allocate(24, 4);
            element.set(segment, 4L, 1L, 11);
            int witness = (int) strided.compareAndExchange(segment, 0L, 2L, 11, 17);
            if (witness != 11 || segment.get(ValueLayout.JAVA_INT, 8) != 17) {
                throw new AssertionError("base offset and stride");
            }
            expect(IllegalArgumentException.class, () -> { int v = (int) element.get(segment, 1L, 0L); });
            expect(IndexOutOfBoundsException.class, () -> { int v = (int) element.get(segment, 0L, 3L); });
            expect(IndexOutOfBoundsException.class, () -> { int v = (int) element.get(segment, 24L, 0L); });
            expect(IllegalArgumentException.class, () -> element.set(segment.asReadOnly(), 4L, 1L, 19));
            Throwable[] failure = new Throwable[1];
            Thread worker = new Thread(() -> {
                try {
                    int v = (int) element.get(segment, 4L, 1L);
                } catch (Throwable t) {
                    failure[0] = t;
                }
            });
            worker.start();
            worker.join();
            if (!(failure[0] instanceof WrongThreadException)) throw new AssertionError("confined segment", failure[0]);
        }
        expect(IllegalStateException.class, () -> { int v = (int) element.get(segment, 4L, 1L); });
    }

    static void expect(Class<? extends RuntimeException> type, Runnable action) {
        try {
            action.run();
        } catch (RuntimeException failure) {
            if (failure.getClass() == type) return;
            throw failure;
        }
        throw new AssertionError("expected " + type.getName());
    }
}
