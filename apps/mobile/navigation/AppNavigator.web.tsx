import { createBrowserApp } from '@react-navigation/web';
import AppContainer from './AppContainer';


AppContainer.path = '';

export default createBrowserApp(AppContainer, { history: 'hash' });
