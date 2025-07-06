/** Test Object interface implementation behavior. */
interface Walker { void walk(); }

interface Swimmer { void swim(); }

class Person implements Walker, Swimmer {
    public void walk() { System.out.println("Walking"); }
    public void swim() { System.out.println("Swimming"); }
}

public class Test {
    public static void main(String[] args) {
        Person p = new Person();
        System.out.println(p instanceof Walker);
        System.out.println(p instanceof Swimmer);
        System.out.println(p instanceof Object);
        Walker w = p;
        System.out.println(w instanceof Person);
        System.out.println(w instanceof Swimmer);
        System.out.println(w instanceof Object);
    }
}
