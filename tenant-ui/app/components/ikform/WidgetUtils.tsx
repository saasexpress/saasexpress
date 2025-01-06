import ToolTipSimple from "components/gold/tooltip-simple";

class WidgetUtils {
  static dot(item: any, d: string) {
    d.split(".").forEach((a) => {
      if (a.indexOf("[") != -1) {
        // it's an array reference
        let root = a.substring(0, a.indexOf("["));
        let ind = a.substring(a.indexOf("[") + 1, a.indexOf("]"));
        if (root == "") {
          item = item[ind];
        } else {
          item = item[root][ind];
        }
      } else {
        item = item ? item[a] : null;
      }
    });
    return item;
  }

  static tooltip(props: any, _?: any) {
    let tooltip = <></>;
    if (props.hasOwnProperty("help") && props.help?.length > 0) {
      const help = props.help;
      tooltip = (
        <ToolTipSimple tooltip={<span>{help}</span>} placement="right">
          <span>
            <i className="fa fa-question-circle"></i>
          </span>
        </ToolTipSimple>
      );
      const clearButton = <></>;
      const invalid = <></>;

      return [tooltip, clearButton, invalid];
    }

    return undefined;
  }

  static setValue(data: any, key: string, value: any, _default = null) {
    const keys = key.split(".");
    let item = data;

    keys.forEach(function (a, i) {
      if (a.indexOf("[") != -1) {
        let root = a.substring(0, a.indexOf("["));
        let ind = a.substring(a.indexOf("[") + 1, a.indexOf("]"));
        if (root == "") {
          item = item[ind];
        } else {
          item = item[root][ind];
        }
      } else {
        if (i == keys.length - 1) {
          if (typeof item == "undefined" || item == null) {
            item = {};
          }

          item[a] = value;
          if (typeof value === "boolean" || typeof value === "object") {
          } else {
            if (value == null || value == "") {
              if (_default == null) {
                delete item[a];
              } else {
                item[a] = _default;
              }
            }
          }
          return;
        }
        if (item.hasOwnProperty(a) && item[a] != null) {
          item = item[a];
        } else {
          item[a] = {};
          item = item[a];
        }
      }
    });
  }
}

export default WidgetUtils;
