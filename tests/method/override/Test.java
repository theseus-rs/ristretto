/** Test method overriding behavior in Java. */
public class Test {
    static class Animal {
        public void speak() {
            System.out.println("Animal speaks");
        }
    }

    static class Dog extends Animal {
        @Override
        public void speak() {
            System.out.println("Dog barks");
        }
    }

    public static void main(String[] args) {
        Animal animal = new Animal();
        Dog dog = new Dog();
        Animal animalDog = new Dog();

        animal.speak();
        dog.speak();
        animalDog.speak();
    }
}
