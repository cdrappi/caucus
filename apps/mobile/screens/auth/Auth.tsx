import React from "react";
import { Button, SafeAreaView, StyleSheet, View } from "react-native";
import Separator from "../../components/Separator";

export default function LoggedOut({ navigation }) {
  return (
    <SafeAreaView style={styles.container}>
      <Separator flex={2} />
      <View style={styles.buttons}>
        <Separator />
        <Button
          title="Register"
          onPress={() => navigation.navigate("Register")}
        />
        <Separator />
        <Button title="Log In" onPress={() => navigation.navigate("Login")} />
        <Separator />
      </View>
      <Separator flex={2} />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
    marginHorizontal: 16
  },
  buttons: {
    flex: 1,
    flexDirection: "row"
  },
  title: {
    textAlign: "center",
    marginVertical: 8,
    justifyContent: "center",
    color: "#000",
    fontSize: 18
  }
});
