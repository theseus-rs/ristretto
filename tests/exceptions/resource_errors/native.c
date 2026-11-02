/* JNI support for bounded native-stack tests; no machine-wide limits are changed. */
#include <jni.h>
#include <stdio.h>
#include <sys/resource.h>
#include <unistd.h>

static struct rlimit saved_limit;
static int limit_active;
static jmethodID callback;
static volatile unsigned char stack_sink;

JNIEXPORT jboolean JNICALL Java_Test_limitAddressSpace(JNIEnv *env, jclass type) {
    (void) env;
    (void) type;
    FILE *status = fopen("/proc/self/statm", "r");
    unsigned long pages;
    if (status == NULL) return JNI_FALSE;
    int parsed = fscanf(status, "%lu", &pages);
    fclose(status);
    long page_size = sysconf(_SC_PAGESIZE);
    if (parsed != 1 || page_size <= 0 || getrlimit(RLIMIT_AS, &saved_limit) != 0) return JNI_FALSE;
    struct rlimit bounded = saved_limit;
    bounded.rlim_cur = (rlim_t) pages * (rlim_t) page_size + 16 * 1024 * 1024;
    if (saved_limit.rlim_cur != RLIM_INFINITY && bounded.rlim_cur > saved_limit.rlim_cur) return JNI_FALSE;
    if (setrlimit(RLIMIT_AS, &bounded) != 0) return JNI_FALSE;
    limit_active = 1;
    return JNI_TRUE;
}

JNIEXPORT jboolean JNICALL Java_Test_restoreAddressSpace(JNIEnv *env, jclass type) {
    (void) env;
    (void) type;
    if (!limit_active || setrlimit(RLIMIT_AS, &saved_limit) != 0) return JNI_FALSE;
    limit_active = 0;
    return JNI_TRUE;
}

JNIEXPORT void JNICALL Java_Test_prepareNative(JNIEnv *env, jclass type) {
    callback = (*env)->GetStaticMethodID(env, type, "nativeCallback", "()V");
}

JNIEXPORT void JNICALL Java_Test_nativeRecurse(JNIEnv *env, jclass type) {
    /* Re-enter Java at every step so HotSpot performs its normal stack-overflow checks.
       Unchecked recursion entirely in C can crash the VM and is not an exception oracle. */
    volatile unsigned char frame[1024];
    frame[0] = 1;
    frame[sizeof(frame) - 1] = 2;
    (*env)->CallStaticVoidMethod(env, type, callback);
    /* Do not make further JNI calls with the pending StackOverflowError. */
    stack_sink = (unsigned char) (frame[0] + frame[sizeof(frame) - 1]);
}
