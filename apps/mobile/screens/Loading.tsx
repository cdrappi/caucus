import { Ionicons } from "@expo/vector-icons";
import { Asset } from "expo-asset";
import * as Font from "expo-font";
import React from "react";
import { ActivityIndicator, AsyncStorage, StatusBar, View } from "react-native";

async function loadResourcesAsync() {
  await Promise.all([
    Asset.loadAsync([
      require("../assets/images/robot-dev.png"),
      require("../assets/images/robot-prod.png")
    ]),
    Font.loadAsync({
      // This is the font that we are using for our tab bar
      ...Ionicons.font,
      // We include SpaceMono because we use it in HomeScreen.js. Feel free to
      // remove this if you are not using it in your app
      "space-mono": require("../assets/fonts/SpaceMono-Regular.ttf")
    })
  ]);
}

export default function AuthLoadingScreen({ navigation }) {
  const _bootstrapAsync = async () => {
    const jwt = await AsyncStorage.getItem("jwt");
    navigation.navigate(jwt ? "App" : "Auth");
  };

  loadResourcesAsync().then(_bootstrapAsync);
  return (
    <View>
      <ActivityIndicator />
      <StatusBar barStyle="default" />
    </View>
  );
}
