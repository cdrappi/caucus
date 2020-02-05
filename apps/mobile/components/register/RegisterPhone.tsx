import React, { useState } from "react";
import { Button, StyleSheet } from "react-native";
import { fetchRegisterPhone } from "../../client/register";
import EnterText from "../../components/EnterText";

export default function RegisterPhone({ navigation, setMessageError }) {
  const [phone, setPhone] = useState("");

  function onPress() {
    return fetchRegisterPhone(phone)
      .then(res => res.json())
      .then(json => {
        if (json.success) {
          console.log(json.phone);
          navigation.navigate("Verify", { phone: json.phone });
        } else {
          setMessageError(json.error);
        }
      })
      .catch(reason => console.log(`Error registering: ${reason}`));
  }

  return (
    <React.Fragment>
      <EnterText
        style={styles.input}
        onChangeText={setPhone}
        value={phone}
        placeholder={"18008675309"}
        keyboardType="phone-pad"
        maxLength={15}
      />
      {/* <Separator /> */}
      <Button title="Submit" onPress={onPress} />
      {/* <Separator /> */}
    </React.Fragment>
  );
}

const styles = StyleSheet.create({
  input: {
    width: "80%",
    height: 80,
    textAlign: "center",
    fontSize: 24,
    backgroundColor: "#000",
    color: "#fff",
    borderRadius: 60
  }
});
