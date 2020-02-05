import React, { useState } from "react";
import {
  AsyncStorage,
  Button,
  SafeAreaView,
  StyleSheet,
  View
} from "react-native";
import { fetchLoginWith } from "../../client/login";
import EnterText from "../../components/EnterText";
import Message from "../../components/Message";
import Separator from "../../components/Separator";

function formatLoginType(loginType) {
  if (loginType === "phone") {
    return "phone number";
  }
  return loginType;
}

function ButtonToggle({ toggleType, loginType, setLoginType }) {
  return (
    <Button
      onPress={() => setLoginType(toggleType)}
      title={toggleType}
      color={toggleType === loginType ? "#0f0" : "#888"}
    />
  );
}

export function ToggleLoginType({ loginType, setLoginType }) {
  return (
    <View style={styles.loginTypes}>
      <Separator />
      <ButtonToggle
        toggleType={"phone"}
        loginType={loginType}
        setLoginType={setLoginType}
      />
      <Separator />
      <ButtonToggle
        toggleType={"username"}
        loginType={loginType}
        setLoginType={setLoginType}
      />
      <Separator />
      <ButtonToggle
        toggleType={"email"}
        loginType={loginType}
        setLoginType={setLoginType}
      />
      <Separator />
    </View>
  );
}

export default function Login(props) {
  const { navigation } = props;
  const [loginType, setLoginType] = useState("phone");
  const [login, setLogin] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState(
    "Log in with phone, username or email"
  );
  const [error, setError] = useState(false);
  const setMsg = (msg, err) => {
    setMessage(msg);
    setError(err);
  };

  async function doLogin() {
    return fetchLoginWith(loginType, login, password)
      .then(res => res.json())
      .then(async json => {
        if (json.success) {
          await AsyncStorage.setItem("jwt", json.token);
          navigation.navigate("App");
        } else {
          setMsg(json.error, true);
        }
      })
      .catch(reason => setMsg(`Error logging in: ${reason}`, true));
  }

  return (
    <SafeAreaView style={styles.container}>
      <Message text={message} flex={2} error={error} />
      <Separator flex={2} />
      <ToggleLoginType loginType={loginType} setLoginType={setLoginType} />
      <Separator flex={2} />
      <EnterText
        onChangeText={setLogin}
        value={login}
        keyboardType={loginType === "phone" ? "phone-pad" : "default"}
        placeholder={"Enter your " + formatLoginType(loginType)}
        placeholderTextColor="#999"
      />
      <Separator flex={2} />
      <EnterText
        onChangeText={setPassword}
        value={password}
        placeholder="Password"
        placeholderTextColor="#999"
        secureTextEntry={true}
      />
      <Separator flex={2} />
      <Button title="Submit" onPress={doLogin} />
      <Separator flex={5} />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
    flexDirection: "column",
    marginHorizontal: 16
  },
  errorContainer: {
    justifyContent: "center"
  },
  loginTypes: {
    flex: 1,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    marginHorizontal: 16
  }
});
