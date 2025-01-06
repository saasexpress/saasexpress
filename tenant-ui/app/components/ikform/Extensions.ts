import Widget from "@components/ikform/Widget";

import Checkbox from "./widgets/Checkbox";
import DateTime from "./widgets/DateTime";
import Heading from "./widgets/Heading";
import Input from "./widgets/Input";
import Sensitive from "./widgets/Sensitive";
import Textarea from "./widgets/Textarea";
import Toggle from "./widgets/Toggle";
import RelativeDate from "./widgets/RelativeDate";

import AutocompleteWidget from "./widgets/Autocomplete";

// import AbstractImage from "@components/legacy/ikform/forms/widgets/AbstractImage.jsx";
// import Actions from "@components/legacy/ikform/forms/widgets/Actions.jsx";
// import CodeEditor from "@components/legacy/ikform/forms/widgets/CodeEditor.jsx";
// import ColorPicker from "@components/legacy/ikform/forms/widgets/ColorPicker.jsx";
// import KeyData from "@components/legacy/ikform/forms/widgets/KeyData.jsx";
// import KeyPair from "@components/legacy/ikform/forms/widgets/KeyPair.jsx";
// import Label from "@components/legacy/ikform/forms/widgets/Label.jsx";
// import Logo from "@components/legacy/ikform/forms/widgets/Logo.jsx";
// import NestableList from "@components/legacy/ikform/forms/widgets/NestableList.jsx";
// import Panel from "@components/legacy/ikform/forms/widgets/Panel.jsx";
// import RadioGroup from "@components/legacy/ikform/forms/widgets/RadioGroup.jsx";
// import RelativeDate from "@components/legacy/ikform/forms/widgets/RelativeDate.jsx";
// import ResultSet from "@components/legacy/ikform/forms/widgets/ResultSet.jsx";
// import Select from "@components/legacy/ikform/forms/widgets/Select.jsx";
// import SelectedFilter from "@components/legacy/ikform/forms/widgets/SelectedFilter.jsx";
// import Sensitive from "@components/legacy/ikform/forms/widgets/Sensitive.jsx";
// import Table from "@components/legacy/ikform/forms/widgets/Table.jsx";
// import Tags from "@components/legacy/ikform/forms/widgets/Tags.jsx";
// import Text from "@components/legacy/ikform/forms/widgets/Text.jsx";
// import CustomWidget from "@components/legacy/ikform/forms/widgets/CustomWidget";
// import TagInputWidget from "@components/legacy/ikform/forms/widgets/tag-input";
// import BackgroundImageWidget from "@components/legacy/ikform/forms/widgets/BackgroundImage";

const AllExtensions = [
  Widget("input", Input),
  Widget("checkbox", Checkbox),
  Widget("datetime", DateTime),
  Widget("heading", Heading),
  Widget("textarea", Textarea),
  Widget("toggle", Toggle),
  Widget("relativedate", RelativeDate),
  Widget("sensitive", Sensitive),

  AutocompleteWidget,

  // Widget("abstract", AbstractImage),
  // Widget("actions", Actions),
  // Widget("codeeditor", CodeEditor),
  // Widget("color", ColorPicker),
  // Widget("datetime", DateTime),
  // Widget("heading", Heading),
  // Widget("input", Input),
  // Widget("keydata", KeyData),
  // Widget("keypair", KeyPair),
  // Widget("label", Label),
  // Widget("logo", Logo),
  // Widget("nestablelist", NestableList),
  // Widget("panel", Panel),
  // Widget("radiogroup", RadioGroup),
  // Widget("relativedate", RelativeDate),
  // Widget("resultset", ResultSet),
  // Widget("select", Select),
  // Widget("selectedfilter", SelectedFilter),
  // Widget("sensitive", Sensitive),
  // Widget("table", Table),
  // Widget("tags", Tags),
  // Widget("textarea", Textarea),
  // Widget("text", Text),
  // Widget("toggle", Toggle),
  // new CustomWidget(),
  // new TagInputWidget(),
  // new BackgroundImageWidget(),
  // new AutocompleteWidget(),
];

export default AllExtensions;
