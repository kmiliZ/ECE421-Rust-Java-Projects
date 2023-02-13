package project1;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.math.BigDecimal;
import java.net.URL;
import java.net.URLConnection;

public class APIFinance {
    private static final String BASE_URL = "https://www.alphavantage.co/query?";
    private final static String apiKey = "8WBJ9AS0QW5XAZYD";

    private static int limitCount = 0;
    private static long startTime = 0;
    private static final long timeLimit = 60000;

    public static BigDecimal getPrice(final String symbol) {
        sleepForLimit();
        BigDecimal price = new BigDecimal(0);
        try {
            URL url = new URL(BASE_URL +
                    "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey);
            URLConnection connection = url.openConnection();
            InputStreamReader inputStream = new InputStreamReader(connection.getInputStream(), "UTF-8");
            BufferedReader bufferedReader = new BufferedReader(inputStream);
            String line;
            while ((line = bufferedReader.readLine()) != null) {
                if (line.contains("price")) {
                    price = new BigDecimal(line.split("\"")[3].trim());
                }
            }
            bufferedReader.close();
        } catch (IOException e) {
            System.out.println("failure sending request");
        }
        return price;
    }

    public static void sleepForLimit() {
        if (limitCount == 0) {
            startTime = System.currentTimeMillis(); // initialized starting time
        }
        limitCount++;
        if (limitCount > 5) { // when it is the six request within a time limit
            try {
                Thread.sleep(timeLimit - (System.currentTimeMillis() - startTime));
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
            limitCount = 1; // this is the first request in the next time limit range
            startTime = System.currentTimeMillis();
        }
    }

}