import React, { useState } from "react";
import { StyleSheet, Text, TouchableOpacity } from "react-native";

function isStrongEnough(password) {
  return password.length >= 8;
}

function submitTitle(password, confirmation, triedOnce) {
  if (isStrongEnough(password)) {
    if (password === confirmation) {
      return "Submit";
    } else if (confirmation === "") {
      return "YOLO Submit";
    } else {
      return "Passwords must match";
    }
  } else if (triedOnce) {
    return "Sorry, not strong enough";
  } else {
    return "Enter a password";
  }
}

export default function SubmitPassword({ password, confirmation, onPress }) {
  const disabled = !(
    isStrongEnough(password) &&
    (confirmation === "" || confirmation === password)
  );
  const [tries, setTries] = useState(0);
  const incTries = () => setTries(tries + 1);
  return (
    <React.Fragment>
      <TouchableOpacity
        activeOpacity={0.5}
        onPress={() => {
          incTries();
          onPress();
        }}
        style={styles.button}
      >
        <Text
          style={[
            styles.buttonText,
            tries > 0 && disabled ? styles.disabledText : styles.enabledText
          ]}
        >
          {submitTitle(password, confirmation, tries > 0)}
        </Text>
      </TouchableOpacity>
    </React.Fragment>
  );
}

const styles = StyleSheet.create({
  button: {
    height: 80,
    textAlign: "center",
    backgroundColor: "#ddd",
    justifyContent: "center",
    alignItems: "center",
    borderRadius: 50
  },
  buttonText: {
    textAlign: "center",
    fontWeight: "bold",
    fontSize: 16
  },
  enabledText: {
    color: "#00f"
  },
  disabledText: {
    color: "#f00"
  }
});
