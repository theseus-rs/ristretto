import java.io.*;
import java.util.*;

/**
 * Tests for java.io.ObjectInputStream and ObjectOutputStream classes
 */
public class Test {
    private static final String TEST_DIR = "test_object_streams";
    private static final String OBJECT_FILE = "serialized_objects.dat";

    public static void main(String[] args) {
        System.out.println("=== ObjectInputStream and ObjectOutputStream Tests ===");

        try {
            // Setup
            File testDir = new File(TEST_DIR);
            cleanup(testDir);
            testDir.mkdir();

            testBasicSerialization();
            testPrimitiveTypes();
            testCollectionSerialization();
            testCustomObjectSerialization();
            testSerializationCompatibility();
            testExceptionHandling();
        } catch (Exception e) {
            System.out.println("ERROR: " + e.getMessage());
            e.printStackTrace();
        } finally {
            cleanup(new File(TEST_DIR));
        }

        System.out.println("=== ObjectStream Tests Complete ===");
    }

    // Simple serializable class for testing
    static class TestPerson implements Serializable {
        private static final long serialVersionUID = 1L;

        private String name;
        private int age;
        private transient String temporaryData; // Won't be serialized

        public TestPerson(String name, int age) {
            this.name = name;
            this.age = age;
            this.temporaryData = "temporary";
        }

        @Override
        public String toString() {
            return "TestPerson{name='" + name + "', age=" + age + ", temporaryData='" + temporaryData + "'}";
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            TestPerson person = (TestPerson) obj;
            return age == person.age && Objects.equals(name, person.name);
        }
    }

    // Class with custom serialization
    static class CustomSerializable implements Serializable {
        private static final long serialVersionUID = 1L;

        private String data;
        private int value;

        public CustomSerializable(String data, int value) {
            this.data = data;
            this.value = value;
        }

        private void writeObject(ObjectOutputStream out) throws IOException {
            System.out.println("Custom writeObject called");
            out.defaultWriteObject();
            out.writeUTF("CUSTOM_MARKER");
        }

        private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
            System.out.println("Custom readObject called");
            in.defaultReadObject();
            String marker = in.readUTF();
            System.out.println("Read custom marker: " + marker);
        }

        @Override
        public String toString() {
            return "CustomSerializable{data='" + data + "', value=" + value + "}";
        }
    }

    private static void testBasicSerialization() throws IOException, ClassNotFoundException {
        System.out.println("--- Basic Serialization Tests ---");

        File objectFile = new File(TEST_DIR, OBJECT_FILE);

        // Create test objects
        TestPerson person1 = new TestPerson("Alice", 30);
        TestPerson person2 = new TestPerson("Bob", 25);
        String testString = "Hello, Serialization!";
        Integer testInteger = 42;

        // Serialize objects
        try (FileOutputStream fos = new FileOutputStream(objectFile);
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {

            oos.writeObject(person1);
            oos.writeObject(person2);
            oos.writeObject(testString);
            oos.writeObject(testInteger);

            System.out.println("Serialized objects to file");
        }

        System.out.println("Serialized file size: " + objectFile.length() + " bytes");

        // Deserialize objects
        try (FileInputStream fis = new FileInputStream(objectFile);
             ObjectInputStream ois = new ObjectInputStream(fis)) {

            TestPerson readPerson1 = (TestPerson) ois.readObject();
            TestPerson readPerson2 = (TestPerson) ois.readObject();
            String readString = (String) ois.readObject();
            Integer readInteger = (Integer) ois.readObject();

            System.out.println("Deserialized objects:");
            System.out.println("  Person 1: " + readPerson1);
            System.out.println("  Person 2: " + readPerson2);
            System.out.println("  String: '" + readString + "'");
            System.out.println("  Integer: " + readInteger);

            // Verify equality
            System.out.println("Person 1 equals: " + person1.equals(readPerson1));
            System.out.println("Person 2 equals: " + person2.equals(readPerson2));
            System.out.println("String equals: " + testString.equals(readString));
            System.out.println("Integer equals: " + testInteger.equals(readInteger));
        }
    }

    private static void testPrimitiveTypes() throws IOException, ClassNotFoundException {
        System.out.println("--- Primitive Types Tests ---");

        File primitiveFile = new File(TEST_DIR, "primitives.dat");

        // Write primitive types
        try (FileOutputStream fos = new FileOutputStream(primitiveFile);
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {

            oos.writeBoolean(true);
            oos.writeByte((byte) 127);
            oos.writeShort((short) 32767);
            oos.writeChar('X');
            oos.writeInt(2147483647);
            oos.writeLong(9223372036854775807L);
            oos.writeFloat(3.14159f);
            oos.writeDouble(2.718281828459045);

            // Also test with wrapper classes
            oos.writeObject(Boolean.FALSE);
            oos.writeObject(Byte.valueOf((byte) -128));
            oos.writeObject(Short.valueOf((short) -32768));
            oos.writeObject(Character.valueOf('Y'));
            oos.writeObject(Integer.valueOf(-2147483648));
            oos.writeObject(Long.valueOf(-9223372036854775808L));
            oos.writeObject(Float.valueOf(-3.14159f));
            oos.writeObject(Double.valueOf(-2.718281828459045));

            System.out.println("Written primitive types and wrapper objects");
        }

        // Read primitive types
        try (FileInputStream fis = new FileInputStream(primitiveFile);
             ObjectInputStream ois = new ObjectInputStream(fis)) {

            boolean boolVal = ois.readBoolean();
            byte byteVal = ois.readByte();
            short shortVal = ois.readShort();
            char charVal = ois.readChar();
            int intVal = ois.readInt();
            long longVal = ois.readLong();
            float floatVal = ois.readFloat();
            double doubleVal = ois.readDouble();

            System.out.println("Read primitive types:");
            System.out.println("  boolean: " + boolVal);
            System.out.println("  byte: " + byteVal);
            System.out.println("  short: " + shortVal);
            System.out.println("  char: '" + charVal + "'");
            System.out.println("  int: " + intVal);
            System.out.println("  long: " + longVal);
            System.out.println("  float: " + floatVal);
            System.out.println("  double: " + doubleVal);

            // Read wrapper objects
            Boolean boolObj = (Boolean) ois.readObject();
            Byte byteObj = (Byte) ois.readObject();
            Short shortObj = (Short) ois.readObject();
            Character charObj = (Character) ois.readObject();
            Integer intObj = (Integer) ois.readObject();
            Long longObj = (Long) ois.readObject();
            Float floatObj = (Float) ois.readObject();
            Double doubleObj = (Double) ois.readObject();

            System.out.println("Read wrapper objects:");
            System.out.println("  Boolean: " + boolObj);
            System.out.println("  Byte: " + byteObj);
            System.out.println("  Short: " + shortObj);
            System.out.println("  Character: '" + charObj + "'");
            System.out.println("  Integer: " + intObj);
            System.out.println("  Long: " + longObj);
            System.out.println("  Float: " + floatObj);
            System.out.println("  Double: " + doubleObj);
        }
    }

    private static void testCollectionSerialization() throws IOException, ClassNotFoundException {
        System.out.println("--- Collection Serialization Tests ---");

        File collectionFile = new File(TEST_DIR, "collections.dat");

        // Create collections
        ArrayList<String> stringList = new ArrayList<>();
        stringList.add("First");
        stringList.add("Second");
        stringList.add("Third");

        HashMap<String, Integer> stringIntMap = new HashMap<>();
        stringIntMap.put("one", 1);
        stringIntMap.put("two", 2);
        stringIntMap.put("three", 3);

        HashSet<TestPerson> personSet = new HashSet<>();
        personSet.add(new TestPerson("Alice", 30));
        personSet.add(new TestPerson("Bob", 25));
        personSet.add(new TestPerson("Charlie", 35));

        // Serialize collections
        try (FileOutputStream fos = new FileOutputStream(collectionFile);
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {

            oos.writeObject(stringList);
            oos.writeObject(stringIntMap);
            oos.writeObject(personSet);

            System.out.println("Serialized collections");
        }

        // Deserialize collections
        try (FileInputStream fis = new FileInputStream(collectionFile);
             ObjectInputStream ois = new ObjectInputStream(fis)) {

            @SuppressWarnings("unchecked")
            ArrayList<String> readStringList = (ArrayList<String>) ois.readObject();
            @SuppressWarnings("unchecked")
            HashMap<String, Integer> readStringIntMap = (HashMap<String, Integer>) ois.readObject();
            @SuppressWarnings("unchecked")
            HashSet<TestPerson> readPersonSet = (HashSet<TestPerson>) ois.readObject();

            System.out.println("Deserialized collections:");
            System.out.println("  String list: " + readStringList);
            System.out.println("  String-Int map: " + readStringIntMap);
            System.out.println("  Person set size: " + readPersonSet.size());

            for (TestPerson person : readPersonSet) {
                System.out.println("    " + person);
            }

            // Verify equality
            System.out.println("List equals: " + stringList.equals(readStringList));
            System.out.println("Map equals: " + stringIntMap.equals(readStringIntMap));
            System.out.println("Set equals: " + personSet.equals(readPersonSet));
        }
    }

    private static void testCustomObjectSerialization() throws IOException, ClassNotFoundException {
        System.out.println("--- Custom Object Serialization Tests ---");

        File customFile = new File(TEST_DIR, "custom_objects.dat");

        // Create custom objects
        CustomSerializable custom1 = new CustomSerializable("Test Data 1", 100);
        CustomSerializable custom2 = new CustomSerializable("Test Data 2", 200);

        // Serialize with custom logic
        try (FileOutputStream fos = new FileOutputStream(customFile);
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {

            oos.writeObject(custom1);
            oos.writeObject(custom2);

            System.out.println("Serialized custom objects");
        }

        // Deserialize with custom logic
        try (FileInputStream fis = new FileInputStream(customFile);
             ObjectInputStream ois = new ObjectInputStream(fis)) {

            CustomSerializable readCustom1 = (CustomSerializable) ois.readObject();
            CustomSerializable readCustom2 = (CustomSerializable) ois.readObject();

            System.out.println("Deserialized custom objects:");
            System.out.println("  Custom 1: " + readCustom1);
            System.out.println("  Custom 2: " + readCustom2);
        }
    }

    private static void testSerializationCompatibility() throws IOException, ClassNotFoundException {
        System.out.println("--- Serialization Compatibility Tests ---");

        File compatFile = new File(TEST_DIR, "compatibility.dat");

        // Test null object serialization
        try (FileOutputStream fos = new FileOutputStream(compatFile);
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {

            oos.writeObject(null);
            oos.writeObject("Not null");
            oos.writeObject(null);

            System.out.println("Serialized null objects");
        }

        try (FileInputStream fis = new FileInputStream(compatFile);
             ObjectInputStream ois = new ObjectInputStream(fis)) {

            Object obj1 = ois.readObject();
            Object obj2 = ois.readObject();
            Object obj3 = ois.readObject();

            System.out.println("Read objects:");
            System.out.println("  Object 1: " + obj1);
            System.out.println("  Object 2: " + obj2);
            System.out.println("  Object 3: " + obj3);
        }

        // Test array serialization
        File arrayFile = new File(TEST_DIR, "arrays.dat");
        try (FileOutputStream fos = new FileOutputStream(arrayFile);
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {

            int[] intArray = {1, 2, 3, 4, 5};
            String[] stringArray = {"Hello", "World", "Array"};
            TestPerson[] personArray = {
                new TestPerson("Array Person 1", 20),
                new TestPerson("Array Person 2", 30)
            };

            oos.writeObject(intArray);
            oos.writeObject(stringArray);
            oos.writeObject(personArray);

            System.out.println("Serialized arrays");
        }

        try (FileInputStream fis = new FileInputStream(arrayFile);
             ObjectInputStream ois = new ObjectInputStream(fis)) {

            int[] readIntArray = (int[]) ois.readObject();
            String[] readStringArray = (String[]) ois.readObject();
            TestPerson[] readPersonArray = (TestPerson[]) ois.readObject();

            System.out.println("Read arrays:");
            System.out.println("  Int array: " + Arrays.toString(readIntArray));
            System.out.println("  String array: " + Arrays.toString(readStringArray));
            System.out.println("  Person array: " + Arrays.toString(readPersonArray));
        }
    }

    private static void testExceptionHandling() {
        System.out.println("--- Exception Handling Tests ---");

        // Test ClassNotFoundException
        try {
            File invalidFile = new File(TEST_DIR, "invalid_class.dat");

            // Create a file with invalid class data
            try (FileOutputStream fos = new FileOutputStream(invalidFile);
                 ObjectOutputStream oos = new ObjectOutputStream(fos)) {
                oos.writeObject("Valid string");
            }

            // Try to read as wrong type (this won't cause ClassNotFoundException)
            try (FileInputStream fis = new FileInputStream(invalidFile);
                 ObjectInputStream ois = new ObjectInputStream(fis)) {
                String validString = (String) ois.readObject();
                System.out.println("Read valid string: " + validString);
            }

        } catch (Exception e) {
            System.out.println("Exception handling test: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }

        // Test non-serializable object
        class NonSerializable {
            private String data = "Cannot serialize this";

            @Override
            public String toString() {
                return "NonSerializable{data='" + data + "'}";
            }
        }

        try {
            File nonSerFile = new File(TEST_DIR, "non_serializable.dat");
            try (FileOutputStream fos = new FileOutputStream(nonSerFile);
                 ObjectOutputStream oos = new ObjectOutputStream(fos)) {

                oos.writeObject(new NonSerializable());
                System.out.println("ERROR: Should have thrown NotSerializableException");

            } catch (NotSerializableException e) {
                System.out.println("Correctly caught NotSerializableException: " + e.getMessage());
            }

        } catch (IOException e) {
            System.out.println("IOException in non-serializable test: " + e.getMessage());
        }

        // Test corrupted stream
        try {
            File corruptFile = new File(TEST_DIR, "corrupt.dat");
            try (FileOutputStream fos = new FileOutputStream(corruptFile)) {
                fos.write("This is not a valid object stream".getBytes());
            }

            try (FileInputStream fis = new FileInputStream(corruptFile);
                 ObjectInputStream ois = new ObjectInputStream(fis)) {

                ois.readObject();
                System.out.println("ERROR: Should have thrown StreamCorruptedException");

            } catch (StreamCorruptedException e) {
                System.out.println("Correctly caught StreamCorruptedException: " + e.getMessage());
            } catch (IOException e) {
                System.out.println("Caught IOException for corrupt stream: " + e.getClass().getSimpleName());
            }

        } catch (Exception e) {
            System.out.println("Corrupt stream test error: " + e.getMessage());
        }
    }

    private static void cleanup(File file) {
        if (file.exists()) {
            if (file.isDirectory()) {
                File[] children = file.listFiles();
                if (children != null) {
                    for (File child : children) {
                        cleanup(child);
                    }
                }
            }
            file.delete();
        }
    }
}
