/** Default interface methods can supply the launch class's instance main. */
interface EntryPoint {
    default void main() {
        System.out.println("default main");
        System.out.println(getClass().getName());
    }
}

class Test implements EntryPoint {
    Test() {
        System.out.println("constructed");
    }
}
