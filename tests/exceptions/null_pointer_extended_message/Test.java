import java.lang.reflect.Method;

public class Test {
    public Object nullObj = null;
    public int[] nullArray = null;

    public static void main(String[] args) {
        System.out.println("Start");
        try {
            Test t = new Test();
            try {
                t.accessMethod();
            } catch (NullPointerException e) {
                StringBuilder sb = new StringBuilder();
                sb.append("accessMethod: ");
                sb.append(e.getMessage());
                System.out.println(sb.toString());
                printExtended(e);
            }
            try {
                t.readField();
            } catch (NullPointerException e) {
                StringBuilder sb = new StringBuilder();
                sb.append("readField: ");
                sb.append(e.getMessage());
                System.out.println(sb.toString());
                printExtended(e);
            }
            try {
                t.arrayLoad();
            } catch (NullPointerException e) {
                StringBuilder sb = new StringBuilder();
                sb.append("arrayLoad: ");
                sb.append(e.getMessage());
                System.out.println(sb.toString());
                printExtended(e);
            }
            try {
                t.arrayLength();
            } catch (NullPointerException e) {
                StringBuilder sb = new StringBuilder();
                sb.append("arrayLength: ");
                sb.append(e.getMessage());
                System.out.println(sb.toString());
                printExtended(e);
            }
        } catch (Throwable t) {
            StringBuilder sb = new StringBuilder();
            sb.append("Caught throwable: ");
            sb.append(t.getClass().getName());
            sb.append(": ");
            sb.append(t.getMessage());
            System.out.println(sb.toString());
            t.printStackTrace();
        }
    }

    private static void printExtended(NullPointerException e) {
        try {
            Method m = NullPointerException.class.getDeclaredMethod("getExtendedNPEMessage");
            m.setAccessible(true);
            String msg = (String) m.invoke(e);
            StringBuilder sb = new StringBuilder();
            sb.append("Extended: ");
            sb.append(msg);
            System.out.println(sb.toString());
        } catch (Exception ex) {
            StringBuilder sb = new StringBuilder();
            sb.append("Reflection failed: ");
            sb.append(ex.getClass().getName());
            sb.append(": ");
            sb.append(ex.getMessage());
            System.out.println(sb.toString());
        }
    }

    public void accessMethod() {
        Object x = nullObj;
        x.toString();
    }

    public void readField() {
         Test t = null;
         Object x = t.nullObj;
    }
    
    public void arrayLoad() {
        int x = nullArray[0];
    }

    public void arrayLength() {
        int x = nullArray.length;
    }
}
