package dev.fitko.fitconnect.zbp;

public class ZBPEnvironment {

  public static final ZBPEnvironment INT_INTERNET =
      new ZBPEnvironment("INT_INTERNET", "https://int.zbp.bund.de");
  public static final ZBPEnvironment INT_NDB =
      new ZBPEnvironment("INT_NDB", "https://int.zbp.bmi.in.bund.de");
  public static final ZBPEnvironment INT_NDB_VN =
      new ZBPEnvironment("INT_NDB_VN", "https://int.zbp.bundid.doi-de.net");
  public static final ZBPEnvironment PROD_NDB =
      new ZBPEnvironment("PROD_NDB", "https://prod.zbp.bmi.in.bund.de");
  public static final ZBPEnvironment PROD_NDB_VN =
      new ZBPEnvironment("PROD_NDB_VN", "https://prod.zbp.bundid.doi-de.net");

  private final String name;
  private final String baseUrl;

  /**
   * This flag is for INTERNAL USAGE ONLY and should be set to FALSE by default. For production
   * usage of the FIT-Connect ZBP-Adapter, the TLS/ZBP mailbox certificate is required in PEM format
   * as a cookie in the header of the HTTP request.
   */
  private final boolean sendCertAsCookie;

  public ZBPEnvironment(final String name, final String baseUrl) {
    this(name, baseUrl, false);
  }

  public ZBPEnvironment(final String name, final String baseUrl, final boolean sendCertAsCookie) {
    this.name = name;
    this.baseUrl = baseUrl;
    this.sendCertAsCookie = sendCertAsCookie;
  }

  public String getName() {
    return name;
  }

  public String getBaseUrl() {
    return baseUrl;
  }

  public boolean isSendCertAsCookie() {
    return sendCertAsCookie;
  }
}
