//import createReactClass from "create-react-class";

//import Reflux from "reflux";
//import AuthStore from "../security/stores/AuthStore";

//import APIErrorHandler from "./APIErrorHandler.jsx";

//import GoogleAnalytics from "./GoogleAnalytics";

// import { useRouter } from "next/router";
// import { useSession } from "next-auth/react";

//import APIErrorHandler from "lib/alerts/APIErrorHandler";
import PageAlert from "../alerts/PageAlert";
//import { useAppContext } from "@/lib/context";

function APICall() {
  function get(
    path: string,
    dispatch: Function,
    callback: any = null,
    cbFailure: any = null,
    contentType = "application/json"
  ) {
    const method = "GET";
    const headers = {
      "Content-Type": contentType,
    };

    // PageAlert.doit({
    //   open: true,
    //   snackbar: true,
    //   severity: "success", // warning, error, info, success
    //   title: "Loading...",
    //   action: { link: "/shell", label: "Go to labels" },
    //   content: ["a", "b"],
    // });

    fetch(path, {
      method,
      headers,
    })
      .then(async (res) => {
        //GoogleAnalytics.restCall ('rest_call_get_success', path);
        if (res.status == 401 || res.status == 403) {
          PageAlert.nextAction({ redirect: "/api/auth/signout" });
          return;
        } else if (res.status != 200) {
          //GoogleAnalytics.restCall ('rest_call_post_fail', path);
          const d = await res.json();
          if (callback) {
            callback(d);
          } else {
            if (cbFailure) {
              cbFailure(d);
            } else {
              console.log("APIErrorHandler.alert(d);");
            }
          }

          //dispatch(`Error (${response.status}) ${d}`, null);
        } else {
          //PageAlert.doit({ open: false, snackbar: false });

          var paging = {};
          const headers = res.headers;
          // headers.forEach((val, key) => {
          //   console.log(key, val);
          // });
          if (headers.has("paging-total-records")) {
            paging = {
              total_records: Number(headers.get("paging-total-records")),
              total_pages: Number(headers.get("paging-total-pages")),
              current_page: Number(headers.get("paging-current-page")),
              page_size: Number(headers.get("paging-page-size")),
            };
          } else {
            paging = {
              total_records: 0,
              total_pages: 0,
              current_page: 0,
              page_size: 25,
            };
          }

          // NOTE: I ONLY HAVE THIS HERE BECAUSE Blueprint Notification Settings
          // RETURNS NOTHING IF THERE ARE NO PREFERNCES.. CHANGE THIS ON API!
          const jss = await res.text();
          try {
            const json = JSON.parse(jss);
            dispatch(json, paging);
          } catch (ex) {
            dispatch({ result: jss }, paging);
          }

          // const jss = await res.json();
          // dispatch(jss, paging);
        }
      })
      .catch((_: Error) => {
        console.log("APIErrorHandler.alert(error);");
      });
  }

  /*
  function handleSubmitReportingError(path, values, dispatch) {
    let url = SMARTADMIN_GLOBALS.apis.apiRootUrl + path;
    let method = "POST";

    let headers = prepareHeaders();

      .catch((error) => {
        //GoogleAnalytics.restCall('rest_call_get_fail', path);
        if (callback) {
          callback(error);
        } else {
          if (cbFailure) {
            cbFailure(error);
          } else {
            console.log("APIErrorHandler.alert(error);");
          }
        }
      });
      headers,
    }).then(async (response) => {
      const d = await response.json();
      console.log("RES = " + response.status);
      console.log(JSON.stringify(d));
      if (response.status != 200) {
        //GoogleAnalytics.restCall ('rest_call_post_fail', path);
        dispatch("Error " + d, null);
      } else {
        //GoogleAnalytics.restCall ('rest_call_post_success', path);
        dispatch(null, d);
      }
    });
  }
*/

  function put(path: string, values: any, dispatch: Function) {
    let method = "PUT";

    let headers = {
      "Content-Type": "application/json",
    };

    fetch(path, {
      method,
      body: typeof values === "string" ? values : JSON.stringify(values),
      headers,
      // success: (d) => {
      //     GoogleAnalytics.restCall ('rest_call_post_success', path);
      //     dispatch(d);
      // },
      // error: (d) => {
      //     GoogleAnalytics.restCall ('rest_call_post_fail', path);
      //     APIErrorHandler.alert(d);
      // }
    })
      .then(async (request) => {
        //GoogleAnalytics.restCall ('rest_call_get_success', path);

        // if (request.getResponseHeader('X-Paging-Total-Records')) {
        //     paging = {
        //         total_records: Number(request.getResponseHeader('X-Paging-Total-Records')),
        //         total_pages: Number(request.getResponseHeader('X-Paging-Total-Pages')),
        //         current_page: Number(request.getResponseHeader('X-Paging-Current-Page')),
        //         page_size: Number(request.getResponseHeader('X-Paging-Page-Size'))
        //     }
        // }
        const jss = await request.text();
        try {
          const json = JSON.parse(jss);
          dispatch(json);
        } catch (e) {
          console.log("Ooops... " + e);
          //APIErrorHandler.alert(ex);
          dispatch({ result: jss });
        }
      })
      .catch((e) => {
        console.log("Ooops... " + e);
        //APIErrorHandler.alert(e);
      });
  }

  function post(path: string, values: any, dispatch: Function) {
    let method = "POST";

    let headers = {
      "Content-Type": "application/json",
    };

    fetch(path, {
      method,
      body: typeof values === "string" ? values : JSON.stringify(values),
      headers,
      // success: (d) => {
      //     GoogleAnalytics.restCall ('rest_call_post_success', path);
      //     dispatch(d);
      // },
      // error: (d) => {
      //     GoogleAnalytics.restCall ('rest_call_post_fail', path);
      //     APIErrorHandler.alert(d);
      // }
    })
      .then(async (request) => {
        //GoogleAnalytics.restCall ('rest_call_get_success', path);

        // if (request.getResponseHeader('X-Paging-Total-Records')) {
        //     paging = {
        //         total_records: Number(request.getResponseHeader('X-Paging-Total-Records')),
        //         total_pages: Number(request.getResponseHeader('X-Paging-Total-Pages')),
        //         current_page: Number(request.getResponseHeader('X-Paging-Current-Page')),
        //         page_size: Number(request.getResponseHeader('X-Paging-Page-Size'))
        //     }
        // }
        const jss = await request.text();
        try {
          const json = JSON.parse(jss);
          dispatch(json);
        } catch (e) {
          console.log("Ooops... " + e);
          //APIErrorHandler.alert(ex);
          dispatch({ result: jss });
        }
      })
      .catch((e) => {
        console.log("Ooops... " + e);
        //APIErrorHandler.alert(e);
      });
  }

  function _delete(path: string, dispatch: Function) {
    let method = "DELETE";

    let headers = {
      "Content-Type": "application/json",
    };

    fetch(path, {
      method,
      headers,
      // success: (d) => {
      //     GoogleAnalytics.restCall ('rest_call_post_success', path);
      //     dispatch(d);
      // },
      // error: (d) => {
      //     GoogleAnalytics.restCall ('rest_call_post_fail', path);
      //     APIErrorHandler.alert(d);
      // }
    })
      .then(async (response) => {
        //GoogleAnalytics.restCall ('rest_call_get_success', path);

        // if (request.getResponseHeader('X-Paging-Total-Records')) {
        //     paging = {
        //         total_records: Number(request.getResponseHeader('X-Paging-Total-Records')),
        //         total_pages: Number(request.getResponseHeader('X-Paging-Total-Pages')),
        //         current_page: Number(request.getResponseHeader('X-Paging-Current-Page')),
        //         page_size: Number(request.getResponseHeader('X-Paging-Page-Size'))
        //     }
        // }
        if (response.status == 204) {
          dispatch(undefined);
        } else if (response.status != 200) {
          throw Error("Error " + response.statusText);
        }
        const body = await response.text();
        dispatch(body);
      })
      .catch((e) => {
        console.log("Ooops... " + e);
        //APIErrorHandler.alert(e);
      });
  }
  /*
  function handlePut(path, values, dispatch) {
    let url = SMARTADMIN_GLOBALS.apis.apiRootUrl + path;
    let method = "PUT";

    let headers = prepareHeaders();

    fetch(url, {
      method,
      body: JSON.stringify(values),
      headers,
      success: (d) => {
        GoogleAnalytics.restCall("rest_call_put_success", path);
        dispatch(d);
      },
      error: (d) => {
        GoogleAnalytics.restCall("rest_call_put_fail", path);
        APIErrorHandler.alert(d);
      },
    }).then(async (response) => {
      const d = await response.text();
      if (response.status != 200) {
        APIErrorHandler.alert(d);
      } else {
        dispatch(d);
      }
    });
  }

  function handleDelete(path, dispatch) {
    let url = SMARTADMIN_GLOBALS.apis.apiRootUrl + path;
    let method = "DELETE";

    let headers = prepareHeaders();

    fetch(url, {
      method,
      headers,
      success: (d) => {
        GoogleAnalytics.restCall("rest_call_delete_success", path);
        dispatch(d);
      },
      error: (d) => {
        GoogleAnalytics.restCall("rest_call_delete_fail", path);
        APIErrorHandler.alert(d);
      },
    })
      .then(async (response) => {
        if (response.status == 204) {
          dispatch(undefined);
        } else if (response.status != 200) {
          throw Exception("Error " + response.statusText);
        }
        const body = await response.text();
        dispatch(body);
      })
      .catch((err) => {
        APIErrorHandler.alert(err);
      });
  }
      */

  return {
    // prepareAuthHeader,
    get,
    put,
    post,
    delete: _delete,
    // handleSubmitReportingError,
    // handleSubmit,
    // handlePut,
    // handleDelete,
  };
}

export default APICall;
