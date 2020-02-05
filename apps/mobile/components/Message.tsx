import React from "react";
import { StyleSheet, Text, View } from "react-native";

export default function Message({ text, flex = 1, error = false }) {
  return (
    <View style={[styles.container, { flex: flex }]}>
      <Text style={[styles.text, { color: error ? "red" : "gray" }]}>
        {text}
      </Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    justifyContent: "center"
  },
  text: {
    textAlign: "center"
  }
});
