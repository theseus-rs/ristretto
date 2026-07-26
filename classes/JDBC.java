import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.Statement;

public class JDBC {
    public static void main(String ... args) throws Exception {
        Class.forName("org.h2.Driver");
        String url = "jdbc:h2:mem:ristretto;DB_CLOSE_DELAY=-1";

        try (Connection connection = DriverManager.getConnection(url, "sa", "");
             Statement statement = connection.createStatement()) {
            statement.executeUpdate(
                "CREATE TABLE people (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL)"
            );
            statement.executeUpdate(
                "INSERT INTO people (id, name) VALUES " +
                "(1, 'Alan Turing'), (2, 'John von Neumann')"
            );

            try (ResultSet resultSet = statement.executeQuery(
                    "SELECT id, name FROM people ORDER BY id")) {
                while (resultSet.next()) {
                    int id = resultSet.getInt("id");
                    String name = resultSet.getString("name");
                    System.out.println(id + "|" + name);
                }
            }
        }
    }
}
