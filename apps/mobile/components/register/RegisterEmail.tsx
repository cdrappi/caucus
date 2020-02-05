import * as EmailValidator from "email-validator";
import React, { useState } from "react";
import { AsyncStorage, StyleSheet, View } from "react-native";
import { fetchRegisterEmail } from "../../client/register";
import EnterText from "../../components/EnterText";
import SubmitPassword from "../../components/util/SubmitPassword";
import Message from "../Message";
import Separator from "../Separator";
import EnterPassword from "../util/EnterPassword";

export default function RegisterEmail({ navigation, setMessageError }) {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmation, setConfirmation] = useState("");
  const [allowSubmit, setAllowSubmit] = useState(true);
  const isEmailValid = email_ => EmailValidator.validate(email_);

  function onPress() {
    if (allowSubmit) {
      setAllowSubmit(false);
      return fetchRegisterEmail(email, password)
        .then(res => res.json())
        .then(async json => {
          if (json.success) {
            console.log(`Successfully registered and created JWT ${json.jwt}`);
            await AsyncStorage.setItem("jwt", json.jwt);
            navigation.navigate("App");
          } else {
            setMessageError(json.error);
          }
        })
        .catch(reason => console.log(`Error registering: ${reason}`));
    }
  }

  function Submit() {
    if (isEmailValid(email)) {
      if (allowSubmit) {
        return (
          <SubmitPassword
            password={password}
            confirmation={confirmation}
            onPress={onPress}
          />
        );
      } else {
        return <Message text={"Processing ..."} flex={5} />;
      }
    } else {
      return <Message text={"Please input a valid email"} flex={5} />;
    }
  }

  return (
    <React.Fragment>
      <Separator />
      <View style={{ flex: 2 }}>
        <View style={[styles.rowContainer]}>
          <Separator />
          <View style={{ flex: 10 }}>
            <EnterText
              onChangeText={setEmail}
              value={email}
              placeholder={"Type in your email"}
              keyboardType={"email-address"}
              maxLength={64}
            />
          </View>
          <Separator />
        </View>
      </View>
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
          <Submit />
          <Separator />
        </View>
        <Separator />
      </View>
    </React.Fragment>
  );
}

const styles = StyleSheet.create({
  rowContainer: {
    flexDirection: "row"
  }
});
