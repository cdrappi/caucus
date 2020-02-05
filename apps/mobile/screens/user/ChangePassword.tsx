import React, { useState } from "react";
import { AsyncStorage, SafeAreaView, StyleSheet, View } from "react-native";
import { fetchChangePassword } from "../../client/user";
import Message from "../../components/Message";
import Separator from "../../components/Separator";
import EnterPassword from "../../components/util/EnterPassword";
import SubmitPassword from "../../components/util/SubmitPassword";

export default function ChangePassword({ navigation }) {
  const [password, setPassword] = useState("");
  const [confirmation, setConfirmation] = useState("");
  const [message, setMessage] = useState(
    "Choose a password 8 characters or longer"
  );

  async function submit() {
    const token = await AsyncStorage.getItem("jwt");
    return fetchChangePassword(password, token)
      .then(res => res.json())
      .then(async json => {
        if (json.success) {
          navigation.navigate("Home");
        } else {
          setMessage(json.error);
          await AsyncStorage.removeItem("jwt");
        }
      })
      .catch(async reason => {
        setMessage(`Error setting password: ${reason}`);
        await AsyncStorage.removeItem("jwt");
        navigation.navigate("Loading");
      });
  }

  return (
    <SafeAreaView style={styles.container}>
      <View style={{ flex: 1 }}>
        <Separator />
        <Message text={message} flex={1} />
        <View style={{ flex: 4 }}>
          <EnterPassword
            password={password}
            confirmation={confirmation}
            setPassword={setPassword}
            setConfirmation={setConfirmation}
          />
        </View>
        <View style={{ flex: 2 }}>
          <Separator />
          <View style={{ flexDirection: "row", flex: 4 }}>
            <Separator />
            <View style={{ flex: 4, width: "100%" }}>
              <SubmitPassword
                password={password}
                confirmation={confirmation}
                onPress={submit}
              />
            </View>
            <Separator />
          </View>
          <Separator />
        </View>
        <Separator />
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    // justifyContent: "center",
    flexDirection: "column"
  }
});
