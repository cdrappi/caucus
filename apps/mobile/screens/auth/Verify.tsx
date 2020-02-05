import React, { useState } from "react";
import { AsyncStorage, Button, SafeAreaView, StyleSheet } from "react-native";
import { fetchRegisterVerifyPhone } from "../../client/register";
import EnterText from "../../components/EnterText";
import Separator from "../../components/Separator";

export default function Verify({ navigation }) {
  const [code, setCode] = useState("");
  const { phone } = navigation.state.params;

  function onPress() {
    return fetchRegisterVerifyPhone(phone, code)
      .then(res => res.json())
      .then(async json => {
        if (json.success) {
          await AsyncStorage.setItem("jwt", json.token);
          navigation.navigate("App");
        } else {
          console.log(`Failed to verify code: ${JSON.stringify(json)}`);
          await AsyncStorage.removeItem("jwt");
          navigation.navigate("Loading");
        }
      })
      .catch(async reason => {
        console.log(`Error verifying code: ${JSON.stringify(reason)}`);
        await AsyncStorage.removeItem("jwt");
        navigation.navigate("Loading");
      });
  }

  return (
    <SafeAreaView style={styles.container}>
      <Separator flex={2} />
      <EnterText
        onChangeText={setCode}
        value={code}
        keyboardInput={"phone-pad"}
      />
      <Separator />
      <Button title="Verify code" onPress={onPress} />
      <Separator flex={2} />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: "column"
  },
  input: {
    height: 80,
    width: "80%",
    borderColor: "#777",
    borderRadius: 60,
    textAlign: "center",
    color: "#fff",
    backgroundColor: "#000",
    fontSize: 24
  }
});
