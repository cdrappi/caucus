import React from 'react';
import { SafeAreaView, StyleSheet, Text } from 'react-native';
import Separator from "../components/Separator";


function LinksScreen() {
  return (
    <SafeAreaView style={styles.container}>
      <Separator/>
       <Text style={{textAlign: 'center'}}>Links</Text>
       <Separator/>
    </SafeAreaView>
  );
}

LinksScreen.navigationOptions = {
  title: 'Links',
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    paddingTop: 15,
    backgroundColor: '#fff',
  },
});

export default React.memo(LinksScreen)
