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
            }
            try {
                t.readField();
            } catch (NullPointerException e) {
                StringBuilder sb = new StringBuilder();
                sb.append("readField: ");
                sb.append(e.getMessage());
                System.out.println(sb.toString());
            }
            try {
                t.arrayLoad();
            } catch (NullPointerException e) {
                StringBuilder sb = new StringBuilder();
                sb.append("arrayLoad: ");
                sb.append(e.getMessage());
                System.out.println(sb.toString());
            }
            try {
                t.arrayLength();
            } catch (NullPointerException e) {
                StringBuilder sb = new StringBuilder();
                sb.append("arrayLength: ");
                sb.append(e.getMessage());
                System.out.println(sb.toString());
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
