import React from "react";
import { StyleSheet, Text } from "react-native";

export default function EmailMessage({ message }) {
  if (!message) {
    return (
      <React.Fragment>
        <Text style={styles.instructions}>Choose an email and password</Text>
        {/* <Separator /> */}
        <Text style={styles.subInstructions}>
          Feel free to skip the password confirmation!
        </Text>
      </React.Fragment>
    );
  }
  return <Text style={styles.instructions}>{message}</Text>;
}

const styles = StyleSheet.create({
  instructions: {
    fontSize: 24
  },
  subInstructions: {
    fontSize: 16,
    color: "#999"
  }
});
