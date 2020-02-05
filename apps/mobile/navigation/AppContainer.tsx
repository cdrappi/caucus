import { Ionicons } from "@expo/vector-icons";
import React from "react";
import { Platform } from "react-native";
import { createAppContainer, createSwitchNavigator } from "react-navigation";
import { createStackNavigator } from "react-navigation-stack";
import { createBottomTabNavigator } from "react-navigation-tabs";
import AuthScreen from "../screens/auth/Auth";
import LoginScreen from "../screens/auth/Login";
import RegisterScreen from "../screens/auth/Register";
import VerifyScreen from "../screens/auth/Verify";
import HomeScreen from "../screens/Home";
import LinksScreen from "../screens/Links";
import LoadingScreen from "../screens/Loading";
import SettingsScreen from "../screens/Settings";
import ChangeEmail from "../screens/user/ChangeEmail";
import ChangePassword from "../screens/user/ChangePassword";
import ChangeUsername from "../screens/user/ChangeUsername";

const platformIcon = icon =>
  Platform.OS === "ios" ? `md-${icon}` : `md-${icon}`;
const getIconBuilder = name => ({ tintColor }) => {
  return <Ionicons name={platformIcon(name)} size={20} color={tintColor} />;
};

const settingsStack = createStackNavigator(
  {
    ChangePassword: ChangePassword,
    ChangeUsername: ChangeUsername,
    ChangeEmail: ChangeEmail,
    SettingsHome: SettingsScreen
  },
  { initialRouteName: "SettingsHome" }
);

const screens = {
  Home: {
    screen: HomeScreen,
    navigationOptions: {
      tabBarLabel: "Home",
      tabBarIcon: getIconBuilder("home")
    }
  },
  Links: {
    screen: LinksScreen,
    navigationOptions: {
      tabBarLabel: "Links",
      tabBarIcon: getIconBuilder("link")
    }
  },
  Settings: {
    screen: settingsStack,
    navigationOptions: {
      tabBarLabel: "Settings",
      tabBarIcon: getIconBuilder("settings")
    }
  }
};
const AppTabNavigator = createBottomTabNavigator(screens, {
  initialRouteName: "Home"
});
const AuthStackNavigator = createStackNavigator({
  Auth: AuthScreen,
  Login: LoginScreen,
  Register: RegisterScreen,
  Verify: VerifyScreen
});

export default createAppContainer(
  createSwitchNavigator(
    {
      Loading: LoadingScreen,
      App: AppTabNavigator,
      Auth: AuthStackNavigator
    },
    {
      initialRouteName: "Loading"
    }
  )
);
