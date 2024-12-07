public final class HelloWorld {

    public static void main(String[] args) {
		int NLOOPS = 100;
		int dividend = 6;
		int divisor = 0;
		int counter = 0;
		int quotient;
		int ii;
		for (ii = 0; ii < NLOOPS; ii++) {
			try {
				quotient = dividend / divisor;
			} catch (Exception ex) {
				counter += 1;
				System.out.printf("catch #%d\n", counter);
			}
		}
		System.out.printf("Finished %d loops of IDIV exceptions\n", NLOOPS);
	}
}
