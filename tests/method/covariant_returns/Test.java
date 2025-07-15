/** Test method covariant return types and bridge methods. */
public class Test {
    // Base class with generic return type
    static class Container<T> {
        protected T value;

        public Container(T value) {
            this.value = value;
        }

        public T getValue() {
            System.out.println("Container.getValue() called");
            return value;
        }

        public Container<T> clone() {
            System.out.println("Container.clone() called");
            return new Container<T>(value);
        }
    }

    // Child class with covariant return types
    static class StringContainer extends Container<String> {
        public StringContainer(String value) {
            super(value);
        }

        @Override
        public String getValue() {
            System.out.println("StringContainer.getValue() called");
            return super.getValue();
        }

        @Override
        public StringContainer clone() {
            System.out.println("StringContainer.clone() called");
            return new StringContainer(this.value);
        }
    }

    // Interface demonstrating covariant returns
    interface Copyable<T> {
        T copy();
    }

    static class Document implements Copyable<Document> {
        private String content;

        public Document(String content) {
            this.content = content;
        }

        @Override
        public Document copy() {
            System.out.println("Document.copy() called");
            return new Document(this.content);
        }

        @Override
        public String toString() {
            return "Document{" + content + "}";
        }
    }

    static class SpecialDocument extends Document {
        private boolean special;

        public SpecialDocument(String content, boolean special) {
            super(content);
            this.special = special;
        }

        @Override
        public SpecialDocument copy() {
            System.out.println("SpecialDocument.copy() called");
            return new SpecialDocument(super.content, this.special);
        }

        @Override
        public String toString() {
            return "SpecialDocument{special=" + special + "}";
        }
    }

    // Demonstrate method with widening return type inheritance
    static class Number {
        public int getNumber() {
            return 42;
        }
    }

    static class ExtendedNumber extends Number {
        @Override
        public int getNumber() {
            return super.getNumber() * 2;
        }

        // Additional method with same name but different signature
        public long getNumber(boolean asLong) {
            return asLong ? getNumber() : 0L;
        }
    }

    public static void main(String[] args) {
        System.out.println("=== Testing Covariant Return Types ===");

        // Test generic covariant returns
        Container<String> container = new StringContainer("Hello");
        String value = container.getValue(); // Returns String, not Object
        System.out.println("Value: " + value);

        StringContainer stringContainer = new StringContainer("World");
        StringContainer cloned = stringContainer.clone(); // Returns StringContainer
        System.out.println("Cloned value: " + cloned.getValue());

        System.out.println("\n=== Testing Interface Covariant Returns ===");

        Document doc = new Document("Original");
        Document docCopy = doc.copy();
        System.out.println("Original: " + doc);
        System.out.println("Copy: " + docCopy);

        SpecialDocument specialDoc = new SpecialDocument("Special", true);
        SpecialDocument specialCopy = specialDoc.copy(); // Returns SpecialDocument
        System.out.println("Special original: " + specialDoc);
        System.out.println("Special copy: " + specialCopy);

        System.out.println("\n=== Testing Polymorphic Covariant Returns ===");

        Document[] docs = {new Document("Doc1"), new SpecialDocument("Doc2", false)};
        for (Document d : docs) {
            Document copy = d.copy(); // Polymorphically calls correct copy method
            System.out.println("Copied: " + copy);
        }

        System.out.println("\n=== Testing Return Type Compatibility ===");

        Number num = new ExtendedNumber();
        int result = num.getNumber();
        System.out.println("Number result: " + result);

        ExtendedNumber extNum = new ExtendedNumber();
        long longResult = extNum.getNumber(true);
        System.out.println("Extended number long result: " + longResult);
    }
}
