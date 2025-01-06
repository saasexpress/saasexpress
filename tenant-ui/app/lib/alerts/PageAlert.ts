import { BehaviorSubject } from "rxjs";

interface PageAlert {
  action: "alert" | "clear" | "next-action" | "none";
  alert?: ActionAlert;
  next?: any;
}

export interface ActionAlert {
  open: boolean;
  snackbar?: boolean;
  title?: string;
  severity?: "info" | "warning" | "error" | "success";
  content?: string[];
  action?: { link: string; label: string };
}

var intentSubject = new BehaviorSubject<PageAlert>({ action: "none" });

var _mod = {
  subject: intentSubject,

  doit: function (p: ActionAlert) {
    intentSubject.next({ action: "alert", alert: p });
  },

  nextAction: function (p: any) {
    intentSubject.next({ action: "next-action", next: p });
  },

  clear: function () {
    intentSubject.next({ action: "clear" });
  },
};

export default _mod;
