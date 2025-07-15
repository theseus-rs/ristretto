public class Test {
    public static class Complex {
        double real, imag;

        Complex(double real, double imag) {
            this.real = real;
            this.imag = imag;
        }

        Complex add(Complex other) {
            return new Complex(this.real + other.real, this.imag + other.imag);
        }

        Complex subtract(Complex other) {
            return new Complex(this.real - other.real, this.imag - other.imag);
        }

        Complex multiply(Complex other) {
            return new Complex(
                this.real * other.real - this.imag * other.imag,
                this.real * other.imag + this.imag * other.real
            );
        }

        @Override
        public String toString() {
            return String.format("%.3f + %.3fi", real, imag);
        }
    }

    public static void fft(Complex[] a) {
        int n = a.length;
        if (n <= 1) return;

        // Divide
        Complex[] even = new Complex[n / 2];
        Complex[] odd = new Complex[n / 2];

        for (int i = 0; i < n / 2; i++) {
            even[i] = a[2 * i];
            odd[i] = a[2 * i + 1];
        }

        // Conquer
        fft(even);
        fft(odd);

        // Combine
        for (int k = 0; k < n / 2; k++) {
            double angle = -2 * Math.PI * k / n;
            Complex t = new Complex(Math.cos(angle), Math.sin(angle)).multiply(odd[k]);

            a[k] = even[k].add(t);
            a[k + n / 2] = even[k].subtract(t);
        }
    }

    public static void ifft(Complex[] a) {
        int n = a.length;

        // Conjugate
        for (int i = 0; i < n; i++) {
            a[i].imag = -a[i].imag;
        }

        // Apply FFT
        fft(a);

        // Conjugate and scale
        for (int i = 0; i < n; i++) {
            a[i].imag = -a[i].imag;
            a[i].real /= n;
            a[i].imag /= n;
        }
    }

    public static void main(String[] args) {
        Complex[] signal = {
            new Complex(1, 0),
            new Complex(2, 0),
            new Complex(3, 0),
            new Complex(4, 0),
            new Complex(0, 0),
            new Complex(0, 0),
            new Complex(0, 0),
            new Complex(0, 0)
        };

        System.out.println("Original signal:");
        for (Complex c : signal) {
            System.out.println(c);
        }

        fft(signal);
        System.out.println("\nAfter FFT:");
        for (Complex c : signal) {
            System.out.println(c);
        }

        ifft(signal);
        System.out.println("\nAfter IFFT (should be original):");
        for (Complex c : signal) {
            System.out.println(c);
        }
    }
}

