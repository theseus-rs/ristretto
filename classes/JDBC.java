import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.Statement;

public class JDBC {
    public static void main(String ... args) throws Exception {
        Class.forName("org.h2.Driver");
        String url = "jdbc:h2:~/test";
        String user = "sa";
        String password = "";

        try (Connection connection = DriverManager.getConnection(url, user, password);
             Statement statement = connection.createStatement();
             ResultSet resultSet = statement.executeQuery("SELECT H2VERSION()")) {

            if (resultSet.next()) {
                String version = resultSet.getString(1);
                System.out.println("H2 Database Version: " + version);
            }
        }
    }
}
