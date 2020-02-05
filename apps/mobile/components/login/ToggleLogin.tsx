import React from "react";
import { Button, StyleSheet, View } from "react-native";
import Separator from "../Separator";

export default function ToggleLogin({ usePhone, setUsePhone, height = 100 }) {
  return (
    <View style={[styles.container, { height: height }]}>
      <View style={{ flex: 1 }} />
      <View style={[{ flexDirection: "row", flex: 5 }]}>
        <Separator />
        <Button
          title="Email"
          onPress={() => setUsePhone(false)}
          color={usePhone ? "gray" : "green"}
        />
        <Separator />
        <Button
          title="Phone"
          onPress={() => setUsePhone(true)}
          color={usePhone ? "green" : "gray"}
        />
        <Separator />
      </View>
      <View style={{ flex: 1 }} />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: "column"
  }
});
