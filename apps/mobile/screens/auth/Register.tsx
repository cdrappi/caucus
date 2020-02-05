import React, { useState } from "react";
import { SafeAreaView, StyleSheet, View } from "react-native";
import ToggleLogin from "../../components/login/ToggleLogin";
import Message from "../../components/Message";
import RegisterEmail from "../../components/register/RegisterEmail";
import RegisterPhone from "../../components/register/RegisterPhone";

export default function Register({ navigation }) {
  const [usePhone, setUsePhone] = useState(false);
  const [message, setMessage] = useState(
    "Register with an email or phone number"
  );
  const [error, setError] = useState(false);
  const setMessageError = msg => {
    setMessage(msg);
    setError(true);
  };

  return (
    <SafeAreaView style={styles.container}>
      <Message text={message} flex={1} />
      <View style={[{ flex: 1 }]}>
        <ToggleLogin usePhone={usePhone} setUsePhone={setUsePhone} />
      </View>
      <View style={[{ flex: 8 }]}>
        {usePhone ? (
          <RegisterPhone
            navigation={navigation}
            setMessageError={setMessageError}
          />
        ) : (
          <RegisterEmail
            navigation={navigation}
            setMessageError={setMessageError}
          />
        )}
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: "column"
  }
});
