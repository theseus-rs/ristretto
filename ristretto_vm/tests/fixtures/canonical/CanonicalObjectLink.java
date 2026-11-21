public final class CanonicalObjectLink {
    int value;
    CanonicalObjectLink next;
    CanonicalObjectLink(int value) { this.value = value; }
    public static int read(int value) {
        CanonicalObjectLink holder = new CanonicalObjectLink(0);
        holder.next = new CanonicalObjectLink(value);
        CanonicalObjectLink retained = holder.next;
        holder.next = null;
        return retained.value;
    }
}
