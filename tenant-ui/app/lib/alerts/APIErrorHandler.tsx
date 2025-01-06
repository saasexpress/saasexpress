import PageAlert from "./PageAlert";

class APIErrorHandler {
  static alert = function (event: any) {
    if (event.status == 403) {
      let msg = "Access Denied";
      PageAlert.doit({
        snackbar: true,
        open: false,
        title: msg,
        content: ["You do not have permission to access this resource."],
        severity: "error",
      });
      return;
    }
    if (event.status == 401) {
      //History.push('/a/login');
      return;
    }
    var message = event.responseJSON
      ? event.responseJSON.message
      : event.responseText;
    if (event.status != 0) {
      let title = "Service Error";
      let msg = "We are facing a problem with connecting to SaaS Express.";
      event.statusText && (msg = msg + "<P>" + event.statusText + "</P>");
      if (event.responseJSON) {
        msg = event.responseJSON.code
          ? event.responseJSON.code
          : event.responseJSON.error
          ? event.responseJSON.error
          : "";
        title = event.responseJSON.category
          ? event.responseJSON.category
          : "Service Error";
      }
      PageAlert.doit({
        snackbar: true,
        open: false,
        title: msg,
        content: [message],
        severity: "error",
      });
    } else {
      let msg = "Unable to connect to SaaS Express.";
      PageAlert.doit({
        snackbar: true,
        open: false,
        title: msg,
        severity: "error",
      });
    }
  };
  static notice = function (event: any) {
    PageAlert.doit({
      snackbar: true,
      open: false,
      title: event.title,
      content: [event.content],
      severity: "info",
    });
  };
}

export default APIErrorHandler;
