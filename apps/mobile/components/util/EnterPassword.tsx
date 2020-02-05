import React, { useState } from "react";
import { Button, StyleSheet, View } from "react-native";
import EnterText from "../../components/EnterText";
import Separator from "../Separator";

export default function EnterPassword({
  password,
  setPassword,
  confirmation,
  setConfirmation
}) {
  const [hidePasswords, setHidePasswords] = useState(true);
  const generateRandomPassword = () => {
    setPassword(`${(Math.random() * 10 ** 9).toFixed(0)}`);
    setHidePasswords(false);
  };
  return (
    <React.Fragment>
      <View style={[styles.container]}>
        <Separator />
        <View style={[styles.rowContainer, { flex: 2 }]}>
          <Separator flex={1} />
          <View style={{ flex: 2 }}>
            <Button
              title={hidePasswords ? "show" : "hide"}
              onPress={() => setHidePasswords(!hidePasswords)}
            />
          </View>
          <Separator />
          <View style={{ flex: 3 }}>
            <Button title="random()" onPress={generateRandomPassword} />
          </View>
          <Separator />
        </View>
        <View style={[styles.rowContainer, { flex: 5 }]}>
          <Separator />
          <View style={[styles.passwordsContainer, { flex: 18 }]}>
            <View style={{ flex: 5 }}>
              <EnterText
                style={styles.input}
                onChangeText={setPassword}
                value={password}
                placeholder={"Password"}
                secureTextEntry={hidePasswords}
              />
            </View>
            <Separator />
            <View style={{ flex: 5 }}>
              <EnterText
                style={styles.input}
                onChangeText={setConfirmation}
                value={confirmation}
                placeholder="Confirm (optional)"
                secureTextEntry={hidePasswords}
              />
            </View>
          </View>
          <Separator />
        </View>
        <Separator />
      </View>
    </React.Fragment>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: "column",
    backgroundColor: "pink"
  },
  rowContainer: {
    flexDirection: "row"
  },
  passwordsContainer: {
    flexDirection: "column"
  },
  iconContainer: {
    justifyContent: "center"
  },
  input: {
    height: 60,
    borderRadius: 30,
    textAlign: "center",
    fontSize: 16
  }
});
