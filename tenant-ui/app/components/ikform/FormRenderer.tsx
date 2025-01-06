import { useEffect, Ref } from "react";

import Emitter from "./ListStore";

import UIElementRenderer from "./UIElementRenderer";

interface FormRendererProps {
  id?: string;
  extensions: any;
  model: any;
  ui: any;
  ref: Ref<any>;
  onChange: any;
  onAction: any;
}

const FormRenderer = (props: FormRendererProps) => {
  const emitter = new Emitter();

  // constructor(props) {
  //   super(props);
  //   this.state = { data: {} };
  //   this.emitter = new Emitter();
  // }

  // UNSAFE_componentWillMount() {
  //   this.emitter = new Emitter()
  // }

  useEffect(() => {
    console.log("FormRenderer / MOUNTING FORM");
    if (props.ui?.hasOwnProperty("selectedData")) {
      const selectedData = props.ui.selectedData || {};
      Object.keys(selectedData).map(function (key) {
        emitter.setList(key, selectedData[key]);
      });
    }
    return () => {
      console.log("FormRenderer / DISPOSE FORM");
    };
  }, []);

  // useEffect(() => {
  //   if (props.ui?.hasOwnProperty("selectedData")) {
  //     const selectedData = props.ui.selectedData || {};
  //     Object.keys(selectedData).map(function (key) {
  //       emitter.setList(key, selectedData[key]);
  //     });
  //   }
  // }, [props.ui.selectedData]);

  // componentDidMount() {
  //   const self = this;
  //   if (self.props.ui.hasOwnProperty("selectedData")) {
  //     const selectedData = self.props.ui.selectedData || {};
  //     Object.keys(selectedData).map(function (key) {
  //       self.emitter.setList(key, selectedData[key]);
  //     });
  //   }
  // }

  // componentDidUpdate() {
  //   const self = this;
  //   const selectedData = self.props.ui.selectedData;
  //   // if (self.props.ui.hasOwnProperty('selectedData')) {
  //   //   console.log('FormRenderer / UPDATING SELECTED...');
  //   //   Object.keys(selectedData).map(function (key) {
  //   //     console.log('FormRenderer / UPDATING SELECTED DATA- ' + key);
  //   //     self.emitter.setList(key, selectedData[key]);
  //   //   });
  //   // }
  // }

  // componentWillUnmount() {
  //   //console.log("DISPOSING FORM");
  //   if (this.emitter) {
  //     //this.emitter.dispose()
  //   }
  // }

  // updateSelectData(key, data) {
  //   this.emitter.setList(key, data);
  // }

  // render() {
  //   // if (!this.props || !this.props.ui) {
  //   //   return <p>nothign</p>
  //   // }
  return (
    <UIElementRenderer
      id={props.id}
      extensions={props.extensions}
      emitter={emitter}
      model={props.model}
      mode={props.ui.mode}
      ui={props.ui.elements}
      onChange={props.onChange}
      onAction={props.onAction}
    />
  );
};

export default FormRenderer;
