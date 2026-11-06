import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.VarHandle;

/** Indirect handles must retain their coordinate and receiver adapters. */
public class Test {
    public static void main(String[] args) {
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
        System.out.println("Indirect VarHandle checks passed");
    }
}
