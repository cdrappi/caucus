import React from "react";
import { StyleSheet, Text } from "react-native";

export default function PhoneMessage({ message }) {
  if (!message) {
    return (
      <React.Fragment>
        <Text style={styles.instructions}>Enter your phone number</Text>
        {/* <Separator /> */}
        <Text style={styles.subInstructions}>starting with country code</Text>
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
