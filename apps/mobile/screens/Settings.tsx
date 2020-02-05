import React from "react";
import { AsyncStorage, Button, SafeAreaView, StyleSheet } from "react-native";
import Separator from "../components/Separator";

function SettingsScreen({ navigation }) {
  const logOut = async () => {
    await AsyncStorage.removeItem("jwt");
    navigation.navigate("Auth");
  };

  return (
    <SafeAreaView style={styles.container}>
      <Separator />
      <Button title="Log Out" onPress={logOut} />
      <Separator />
      <Button
        title="Change Password"
        onPress={() => navigation.navigate("ChangePassword")}
      />
      <Separator />
      <Button
        title="Change Email"
        onPress={() => navigation.navigate("ChangeEmail")}
      />
      <Separator />
      <Button
        title="Change Username"
        onPress={() => navigation.navigate("ChangeUsername")}
      />
      <Separator />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    paddingTop: 15,
    backgroundColor: "#fff"
  }
});

export default SettingsScreen;
