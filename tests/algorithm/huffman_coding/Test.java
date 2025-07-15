import java.util.*;

public class Test {
    static class HuffmanNode implements Comparable<HuffmanNode> {
        char character;
        int frequency;
        HuffmanNode left, right;

        HuffmanNode(char character, int frequency) {
            this.character = character;
            this.frequency = frequency;
        }

        HuffmanNode(int frequency, HuffmanNode left, HuffmanNode right) {
            this.frequency = frequency;
            this.left = left;
            this.right = right;
        }

        @Override
        public int compareTo(HuffmanNode other) {
            return Integer.compare(this.frequency, other.frequency);
        }
    }

    public static HuffmanNode buildHuffmanTree(String text) {
        Map<Character, Integer> frequencyMap = new HashMap<>();
        for (char c : text.toCharArray()) {
            frequencyMap.put(c, frequencyMap.getOrDefault(c, 0) + 1);
        }

        PriorityQueue<HuffmanNode> pq = new PriorityQueue<>();
        for (Map.Entry<Character, Integer> entry : frequencyMap.entrySet()) {
            pq.offer(new HuffmanNode(entry.getKey(), entry.getValue()));
        }

        while (pq.size() > 1) {
            HuffmanNode left = pq.poll();
            HuffmanNode right = pq.poll();
            HuffmanNode parent = new HuffmanNode(left.frequency + right.frequency, left, right);
            pq.offer(parent);
        }

        return pq.poll();
    }

    public static Map<Character, String> generateCodes(HuffmanNode root) {
        Map<Character, String> codes = new HashMap<>();
        generateCodesHelper(root, "", codes);
        return codes;
    }

    private static void generateCodesHelper(HuffmanNode node, String code, Map<Character, String> codes) {
        if (node == null) return;

        if (node.left == null && node.right == null) {
            codes.put(node.character, code.isEmpty() ? "0" : code);
            return;
        }

        generateCodesHelper(node.left, code + "0", codes);
        generateCodesHelper(node.right, code + "1", codes);
    }

    public static String encode(String text, Map<Character, String> codes) {
        StringBuilder encoded = new StringBuilder();
        for (char c : text.toCharArray()) {
            encoded.append(codes.get(c));
        }
        return encoded.toString();
    }

    public static String decode(String encoded, HuffmanNode root) {
        StringBuilder decoded = new StringBuilder();
        HuffmanNode current = root;

        for (char bit : encoded.toCharArray()) {
            if (bit == '0') {
                current = current.left;
            } else {
                current = current.right;
            }

            if (current.left == null && current.right == null) {
                decoded.append(current.character);
                current = root;
            }
        }

        return decoded.toString();
    }

    public static void main(String[] args) {
        String text = "ABRACADABRA";
        System.out.println("Original text: " + text);
        System.out.println("Original size: " + text.length() * 8 + " bits");

        HuffmanNode root = buildHuffmanTree(text);
        Map<Character, String> codes = generateCodes(root);

        System.out.println("\nHuffman Codes:");
        for (Map.Entry<Character, String> entry : codes.entrySet()) {
            System.out.println(entry.getKey() + ": " + entry.getValue());
        }

        String encoded = encode(text, codes);
        System.out.println("\nEncoded: " + encoded);
        System.out.println("Encoded size: " + encoded.length() + " bits");
        System.out.println("Compression ratio: " + (double)encoded.length() / (text.length() * 8));

        String decoded = decode(encoded, root);
        System.out.println("\nDecoded: " + decoded);
        System.out.println("Successful: " + text.equals(decoded));
    }
}
