/** Test Object class inheritance behavior. */
class Animal {}

class Dog extends Animal {}

public class Test {
    public static void main(String[] args) {
        Animal animal = new Animal();
        Dog dog = new Dog();
        System.out.println(dog instanceof Dog);
        System.out.println(dog instanceof Animal);
        System.out.println(dog instanceof Object);
        System.out.println(animal instanceof Dog);
        System.out.println(animal instanceof Animal);
        System.out.println(animal instanceof Object);
    }
}
