import React from "react";

import { observer } from "mobx-react";
import { runInAction, observable } from "mobx";

export let listsState = observable.map({});

class ListStore {
  setList = function (name: string, data: any) {
    runInAction(() => {
      listsState.set(name, data);
    });
  };
}

const ListWatcherDisplay = observer(
  class ListWatcherDisplay extends React.Component<any> {
    render() {
      const keyList = Array.from(this.props.lists.keys())
        .map((k: any) => k)
        .join(", ");
      return <div>Watched items: {keyList}</div>;
    }
  }
);

export const ListWatcher = () => <ListWatcherDisplay lists={listsState} />;

export default ListStore;
