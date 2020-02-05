import React from "react";
import { StyleSheet, TextInput } from "react-native";

export default function EnterText({
  onChangeText,
  value,
  placeholder = "",
  secureTextEntry = false,
  style = {},
  ...extraProps
}) {
  return (
    <TextInput
      style={[styles.input, { ...style }]}
      onChangeText={onChangeText}
      value={value}
      autoCapitalize={"none"}
      placeholder={placeholder}
      placeholderTextColor={"#888"}
      secureTextEntry={secureTextEntry}
      {...extraProps}
    />
  );
}
const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: "column"
  },
  input: {
    height: 70,
    width: "100%",
    borderRadius: 60,
    textAlign: "center",
    fontSize: 24,
    borderColor: "#888",
    backgroundColor: "#ddd"
  }
});
