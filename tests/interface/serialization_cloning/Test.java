/** Test interfaces with serialization and cloning */
import java.io.*;

interface SerializableInterface extends Serializable {
    void process();
    String getData();
}

interface CloneableInterface extends Cloneable {
    void setValue(int value);
    int getValue();
    CloneableInterface createCopy();
}

class SerializableImpl implements SerializableInterface {
    private static final long serialVersionUID = 1L;
    private String data;
    private transient int transientValue = 100;

    public SerializableImpl(String data) {
        this.data = data;
    }

    public void process() {
        System.out.println("Processing data: " + data);
        System.out.println("Transient value: " + transientValue);
    }

    public String getData() {
        return data;
    }

    private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
        in.defaultReadObject();
        transientValue = 200; // Reset transient field after deserialization
    }
}

class CloneableImpl implements CloneableInterface {
    private int value;
    private String name;

    public CloneableImpl(int value, String name) {
        this.value = value;
        this.name = name;
    }

    public void setValue(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public CloneableInterface createCopy() {
        try {
            return (CloneableInterface) this.clone();
        } catch (CloneNotSupportedException e) {
            System.out.println("Clone not supported: " + e.getMessage());
            return null;
        }
    }

    @Override
    protected Object clone() throws CloneNotSupportedException {
        return super.clone();
    }

    @Override
    public String toString() {
        return "CloneableImpl{value=" + value + ", name='" + name + "'}";
    }
}

class BothImpl implements SerializableInterface, CloneableInterface {
    private static final long serialVersionUID = 1L;
    private String data;
    private int value;

    public BothImpl(String data, int value) {
        this.data = data;
        this.value = value;
    }

    public void process() {
        System.out.println("BothImpl processing: " + data + ", value: " + value);
    }

    public String getData() {
        return data;
    }

    public void setValue(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public CloneableInterface createCopy() {
        try {
            return (CloneableInterface) this.clone();
        } catch (CloneNotSupportedException e) {
            System.out.println("Clone not supported: " + e.getMessage());
            return null;
        }
    }

    @Override
    protected Object clone() throws CloneNotSupportedException {
        return super.clone();
    }

    @Override
    public String toString() {
        return "BothImpl{data='" + data + "', value=" + value + "}";
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Interface Serialization and Cloning Test ===");

        // Test serialization
        SerializableImpl serializable = new SerializableImpl("test data");
        serializable.process();

        try {
            // Serialize
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            ObjectOutputStream oos = new ObjectOutputStream(baos);
            oos.writeObject(serializable);
            oos.close();

            System.out.println("Object serialized successfully");

            // Deserialize
            ByteArrayInputStream bais = new ByteArrayInputStream(baos.toByteArray());
            ObjectInputStream ois = new ObjectInputStream(bais);
            SerializableInterface deserialized = (SerializableInterface) ois.readObject();
            ois.close();

            System.out.println("Object deserialized successfully");
            deserialized.process();
            System.out.println("Deserialized data: " + deserialized.getData());

        } catch (Exception e) {
            System.out.println("Serialization error: " + e.getMessage());
        }

        // Test cloning
        CloneableImpl cloneable = new CloneableImpl(42, "original");
        System.out.println("Original: " + cloneable);

        CloneableInterface cloned = cloneable.createCopy();
        if (cloned != null) {
            System.out.println("Cloned: " + cloned);

            // Modify original and verify clone is separate
            cloneable.setValue(99);
            System.out.println("After modifying original:");
            System.out.println("Original: " + cloneable);
            System.out.println("Cloned: " + cloned);
        }

        // Test object implementing both interfaces
        BothImpl both = new BothImpl("both data", 123);
        both.process();

        // Test instanceof with multiple interfaces
        System.out.println("both instanceof SerializableInterface: " + (both instanceof SerializableInterface));
        System.out.println("both instanceof CloneableInterface: " + (both instanceof CloneableInterface));
        System.out.println("both instanceof Serializable: " + (both instanceof Serializable));
        System.out.println("both instanceof Cloneable: " + (both instanceof Cloneable));

        // Test cloning of object with both interfaces
        CloneableInterface bothCloned = both.createCopy();
        if (bothCloned != null) {
            System.out.println("Both interface object cloned: " + bothCloned);

            // Test that cloned object still implements both interfaces
            if (bothCloned instanceof SerializableInterface) {
                ((SerializableInterface) bothCloned).process();
            }
        }

        // Test serialization of object with both interfaces
        try {
            ByteArrayOutputStream baos2 = new ByteArrayOutputStream();
            ObjectOutputStream oos2 = new ObjectOutputStream(baos2);
            oos2.writeObject(both);
            oos2.close();

            ByteArrayInputStream bais2 = new ByteArrayInputStream(baos2.toByteArray());
            ObjectInputStream ois2 = new ObjectInputStream(bais2);
            Object deserializedBoth = ois2.readObject();
            ois2.close();

            System.out.println("Both interfaces object serialized/deserialized: " + deserializedBoth);

            if (deserializedBoth instanceof SerializableInterface) {
                ((SerializableInterface) deserializedBoth).process();
            }

        } catch (Exception e) {
            System.out.println("Both interfaces serialization error: " + e.getMessage());
        }

        System.out.println("Serialization and cloning tests completed");
    }
}
