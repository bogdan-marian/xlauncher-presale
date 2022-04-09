import React from 'react';
import {
  ChakraProvider,
  Box,
  Text,
  Link,
  VStack,
  Code,
  Grid,
  HStack,
} from '@chakra-ui/react';
import { ColorModeSwitcher } from './ColorModeSwitcher';
import { Logo } from './Logo';
import theme from './theme/defaultDark';
import ScratchBare from './components/scratchBare';
import BodyComp from './components/bodyComp';
import LocalStorageCountdown from './components/localStorageCountdown';
import { Outlet } from 'react-router-dom';
import { DappProvider } from '@elrondnetwork/dapp-core';
import '@elrondnetwork/dapp-core/build/index.css';
import './styles.css';

const environment = 'devnet';

function App() {
  return (
    <ChakraProvider theme={theme}>
      <DappProvider
        environment={environment}
        customNetworkConfig={{ name: 'customConfig', apiTimeout: 6000 }}
        completedTransactionsDelay={200}
      >
        <Grid>
          <ScratchBare />
          <VStack>
            <Outlet />
          </VStack>
          <VStack>
            <HStack>
              <LocalStorageCountdown/>
            </HStack> 
            <HStack>
              <BodyComp />
            </HStack>    
          </VStack>
        
        </Grid>
      </DappProvider>
    </ChakraProvider>
  );
}

export default App;
