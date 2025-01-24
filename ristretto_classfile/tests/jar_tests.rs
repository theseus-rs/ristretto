mod utilities;

use reqwest::Client;
use std::error::Error;

async fn verify_jar(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let jar_bytes = client.get(url).send().await?.bytes().await?.to_vec();
    utilities::jar::verify(jar_bytes)?;
    Ok(())
}

#[tokio::test]
async fn test_apache_httpclient() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/apache/httpcomponents/client5/httpclient5/5.3.1/httpclient5-5.3.1.jar").await
}

#[tokio::test]
async fn test_aws_mysql() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/software/aws/rds/aws-mysql-jdbc/1.1.15/aws-mysql-jdbc-1.1.15.jar").await
}

#[tokio::test]
async fn test_cassandra() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/apache/cassandra/cassandra-all/4.1.5/cassandra-all-4.1.5.jar").await
}

#[tokio::test]
async fn test_clojure() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/clojure/clojure/1.11.3/clojure-1.11.3.jar").await
}

#[tokio::test]
async fn test_commons_io() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/commons-io/commons-io/2.16.1/commons-io-2.16.1.jar")
        .await
}

#[tokio::test]
async fn test_commons_lang() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/apache/commons/commons-lang3/3.14.0/commons-lang3-3.14.0.jar").await
}

#[tokio::test]
async fn test_derby() -> Result<(), Box<dyn Error>> {
    verify_jar(
        "https://repo1.maven.org/maven2/org/apache/derby/derby/10.17.1.0/derby-10.17.1.0.jar",
    )
    .await
}

#[tokio::test]
async fn test_gson() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/com/google/code/gson/gson/2.11.0/gson-2.11.0.jar")
        .await
}

#[tokio::test]
async fn test_jakarta_servlet() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/jakarta/servlet/jakarta.servlet-api/6.1.0/jakarta.servlet-api-6.1.0.jar").await
}

#[tokio::test]
async fn test_jackson_databind() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/com/fasterxml/jackson/core/jackson-databind/2.17.1/jackson-databind-2.17.1.jar").await
}

#[tokio::test]
async fn test_jtds() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/net/sourceforge/jtds/jtds/1.2.2/jtds-1.2.2.jar")
        .await?;
    verify_jar("https://repo1.maven.org/maven2/net/sourceforge/jtds/jtds/1.3.1/jtds-1.3.1.jar")
        .await
}

#[tokio::test]
async fn test_junit() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/junit/jupiter/junit-jupiter-api/5.10.2/junit-jupiter-api-5.10.2.jar").await
}

#[tokio::test]
async fn test_kotlin() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/jetbrains/kotlin/kotlin-stdlib/2.0.0/kotlin-stdlib-2.0.0.jar").await
}

#[tokio::test]
async fn test_log4j() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/apache/logging/log4j/log4j-core/2.23.1/log4j-core-2.23.1.jar").await
}

#[tokio::test]
async fn test_lombok() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/projectlombok/lombok/1.18.32/lombok-1.18.32.jar")
        .await
}

#[tokio::test]
async fn test_h2() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/com/h2database/h2/2.3.232/h2-2.3.232.jar").await
}

#[tokio::test]
async fn test_mariadb() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/mariadb/jdbc/mariadb-java-client/3.4.0/mariadb-java-client-3.4.0.jar").await
}

#[tokio::test]
async fn test_mockito() -> Result<(), Box<dyn Error>> {
    verify_jar(
        "https://repo1.maven.org/maven2/org/mockito/mockito-core/5.12.0/mockito-core-5.12.0.jar",
    )
    .await
}

#[tokio::test]
async fn test_mysql() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/com/mysql/mysql-connector-j/8.4.0/mysql-connector-j-8.4.0.jar").await
}

#[tokio::test]
async fn test_ojdbc() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/com/oracle/database/jdbc/ojdbc11/23.4.0.24.05/ojdbc11-23.4.0.24.05.jar").await
}

#[tokio::test]
async fn test_postgresql() -> Result<(), Box<dyn Error>> {
    verify_jar(
        "https://repo1.maven.org/maven2/org/postgresql/postgresql/42.7.3/postgresql-42.7.3.jar",
    )
    .await
}

#[tokio::test]
async fn test_scala3() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/scala-lang/scala3-library_3/3.4.2/scala3-library_3-3.4.2.jar").await
}

#[tokio::test]
async fn test_slf4j() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/slf4j/slf4j-api/2.0.13/slf4j-api-2.0.13.jar")
        .await
}

#[tokio::test]
async fn test_snowflake() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/net/snowflake/snowflake-jdbc/3.16.1/snowflake-jdbc-3.16.1.jar").await
}

#[tokio::test]
async fn test_spring_boot() -> Result<(), Box<dyn Error>> {
    verify_jar("https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar").await
}

#[tokio::test]
async fn test_sqlite() -> Result<(), Box<dyn Error>> {
    verify_jar(
        "https://repo1.maven.org/maven2/org/xerial/sqlite-jdbc/3.46.0.0/sqlite-jdbc-3.46.0.0.jar",
    )
    .await
}
