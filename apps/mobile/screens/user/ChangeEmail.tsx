import React, { useState } from "react";
import {
  AsyncStorage,
  SafeAreaView,
  StyleSheet,
  Text,
  TextInput,
  TouchableOpacity
} from "react-native";
import { fetchChangeEmail } from "../../client/user";
import Separator from "../../components/Separator";

export default function ChangeEmail({ navigation }) {
  const [email, setEmail] = useState("");
  const [message, setMessage] = useState("Choose a new email");

  async function submit() {
    const token = await AsyncStorage.getItem("jwt");
    return fetchChangeEmail(email, token)
      .then(res => res.json())
      .then(async json => {
        if (json.success) {
          navigation.navigate("Home");
        } else {
          setMessage(json.error);
        }
      })
      .catch(async reason => {
        setMessage(`Error changing email: ${reason}`);
      });
  }

  return (
    <SafeAreaView style={styles.container}>
      <Separator flex={4} />
      <Text style={styles.message}>{message}</Text>
      <Separator flex={2} />
      <TextInput
        style={styles.input}
        onChangeText={setEmail}
        value={email}
        autoCapitalize={"none"}
        placeholder={"Email"}
        placeholderTextColor="#aaa"
      />
      <Separator flex={1} />
      <TouchableOpacity
        activeOpacity={0.5}
        onPress={submit}
        style={styles.button}
      >
        <Text style={styles.buttonText}>Submit</Text>
      </TouchableOpacity>
      <Separator flex={3} />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
    flexDirection: "column"
  },
  input: {
    height: 50,
    width: "80%",
    borderColor: "#777",
    borderRadius: 60,
    textAlign: "center",
    color: "#fff",
    backgroundColor: "#000",
    fontSize: 16
  },
  passwordsContainer: {
    flex: 4,
    flexDirection: "column"
  },
  showPasswordsContainer: {
    flex: 1
  },
  message: {
    fontSize: 16
  },
  button: {
    width: "60%",
    height: 40,
    textAlign: "center",
    backgroundColor: "#ddd",
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    borderRadius: 50
  },
  buttonText: {
    textAlign: "center",
    fontWeight: "bold",
    fontSize: 16
  }
});
