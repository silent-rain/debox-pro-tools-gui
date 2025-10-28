import { StrictMode } from 'react';
import { BrowserRouter } from 'react-router';
import { RouterGuard } from './routes';

function App() {
  console.log('App');
  return (
    <>
      <StrictMode>
        <BrowserRouter>
          <RouterGuard />
        </BrowserRouter>
      </StrictMode>
    </>
  );
}

export default App;
