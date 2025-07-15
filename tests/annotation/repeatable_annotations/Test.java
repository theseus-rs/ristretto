import java.lang.annotation.*;

// Container annotation for repeatables
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD})
@interface Schedules {
    Schedule[] value();
}

// Repeatable annotation
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.TYPE, ElementType.METHOD, ElementType.FIELD})
@Repeatable(Schedules.class)
@interface Schedule {
    String dayOfMonth() default "first";
    String dayOfWeek() default "Mon";
    int hour() default 12;
}

// Another repeatable annotation
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@interface Authors {
    Author[] value();
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@Repeatable(Authors.class)
@interface Author {
    String name();
    String email() default "";
}

@Schedule(dayOfMonth = "last")
@Schedule(dayOfWeek = "Fri", hour = 23)
@Schedule(dayOfMonth = "15th", dayOfWeek = "Wed", hour = 10)
public class Test {

    @Schedule(hour = 8)
    @Schedule(hour = 14)
    private String scheduledField;

    @Schedule(dayOfWeek = "Mon", hour = 9)
    @Schedule(dayOfWeek = "Wed", hour = 9)
    @Schedule(dayOfWeek = "Fri", hour = 9)
    @Author(name = "Alice", email = "alice@example.com")
    @Author(name = "Bob")
    @Author(name = "Charlie", email = "charlie@example.com")
    public void scheduledMethod() {
    }

    // Method with single repeatable annotation
    @Schedule(dayOfWeek = "Sat", hour = 16)
    public void singleScheduleMethod() {
    }

    public static void main(String[] args) {
        System.out.println("=== Repeatable Annotations Test ===");

        Class<?> clazz = Test.class;

        // Test class-level repeatable annotations
        System.out.println("=== Class Annotations ===");

        // Method 1: Get individual repeated annotations
        Schedule[] classSchedules = clazz.getAnnotationsByType(Schedule.class);
        System.out.println("Class Schedule annotations count: " + classSchedules.length);
        for (int i = 0; i < classSchedules.length; i++) {
            Schedule schedule = classSchedules[i];
            System.out.println("  Schedule " + (i + 1) + ": dayOfMonth=" + schedule.dayOfMonth() +
                             ", dayOfWeek=" + schedule.dayOfWeek() + ", hour=" + schedule.hour());
        }

        // Method 2: Get container annotation
        Schedules schedulesContainer = clazz.getAnnotation(Schedules.class);
        if (schedulesContainer != null) {
            System.out.println("Container annotation found with " + schedulesContainer.value().length + " schedules");
        }

        // Method 3: Get declared annotations (shows container)
        Annotation[] declaredAnnotations = clazz.getDeclaredAnnotations();
        System.out.println("Declared annotations count: " + declaredAnnotations.length);
        for (Annotation ann : declaredAnnotations) {
            System.out.println("  " + ann.annotationType().getSimpleName());
        }

        // Test field-level repeatable annotations
        System.out.println("\n=== Field Annotations ===");
        try {
            java.lang.reflect.Field field = clazz.getDeclaredField("scheduledField");
            Schedule[] fieldSchedules = field.getAnnotationsByType(Schedule.class);
            System.out.println("Field Schedule annotations count: " + fieldSchedules.length);
            for (int i = 0; i < fieldSchedules.length; i++) {
                Schedule schedule = fieldSchedules[i];
                System.out.println("  Field Schedule " + (i + 1) + ": dayOfMonth=" + schedule.dayOfMonth() +
                                 ", dayOfWeek=" + schedule.dayOfWeek() + ", hour=" + schedule.hour());
            }
        } catch (Exception e) {
            System.out.println("Error accessing field: " + e.getMessage());
        }

        // Test method-level repeatable annotations
        System.out.println("\n=== Method Annotations ===");
        try {
            java.lang.reflect.Method method = clazz.getDeclaredMethod("scheduledMethod");

            // Test Schedule annotations
            Schedule[] methodSchedules = method.getAnnotationsByType(Schedule.class);
            System.out.println("Method Schedule annotations count: " + methodSchedules.length);
            for (int i = 0; i < methodSchedules.length; i++) {
                Schedule schedule = methodSchedules[i];
                System.out.println("  Method Schedule " + (i + 1) + ": dayOfMonth=" + schedule.dayOfMonth() +
                                 ", dayOfWeek=" + schedule.dayOfWeek() + ", hour=" + schedule.hour());
            }

            // Test Author annotations
            Author[] methodAuthors = method.getAnnotationsByType(Author.class);
            System.out.println("Method Author annotations count: " + methodAuthors.length);
            for (int i = 0; i < methodAuthors.length; i++) {
                Author author = methodAuthors[i];
                System.out.println("  Author " + (i + 1) + ": name=" + author.name() +
                                 ", email=" + author.email());
            }

            // Test container annotations directly
            Schedules methodSchedulesContainer = method.getAnnotation(Schedules.class);
            Authors methodAuthorsContainer = method.getAnnotation(Authors.class);
            System.out.println("Method has Schedules container: " + (methodSchedulesContainer != null));
            System.out.println("Method has Authors container: " + (methodAuthorsContainer != null));

        } catch (Exception e) {
            System.out.println("Error accessing scheduledMethod: " + e.getMessage());
        }

        // Test single repeatable annotation (should not create container)
        System.out.println("\n=== Single Repeatable Annotation ===");
        try {
            java.lang.reflect.Method singleMethod = clazz.getDeclaredMethod("singleScheduleMethod");

            Schedule[] singleSchedules = singleMethod.getAnnotationsByType(Schedule.class);
            System.out.println("Single method Schedule annotations count: " + singleSchedules.length);

            // Check if container is created for single annotation
            Schedules singleContainer = singleMethod.getAnnotation(Schedules.class);
            Schedule directSchedule = singleMethod.getAnnotation(Schedule.class);

            System.out.println("Has Schedules container: " + (singleContainer != null));
            System.out.println("Has direct Schedule: " + (directSchedule != null));

            if (directSchedule != null) {
                System.out.println("Direct Schedule: dayOfWeek=" + directSchedule.dayOfWeek() +
                                 ", hour=" + directSchedule.hour());
            }

        } catch (Exception e) {
            System.out.println("Error accessing singleScheduleMethod: " + e.getMessage());
        }

        // Test difference between getAnnotations and getAnnotationsByType
        System.out.println("\n=== Annotation Retrieval Methods Comparison ===");

        // getAnnotations returns container, not individual repeated annotations
        Annotation[] allAnnotations = clazz.getAnnotations();
        System.out.println("getAnnotations() count: " + allAnnotations.length);
        for (Annotation ann : allAnnotations) {
            System.out.println("  " + ann.annotationType().getSimpleName());
        }

        // getAnnotationsByType returns individual repeated annotations
        Schedule[] allSchedules = clazz.getAnnotationsByType(Schedule.class);
        System.out.println("getAnnotationsByType(Schedule.class) count: " + allSchedules.length);
    }
}
