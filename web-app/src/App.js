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

const environment = 'devnet';

function App() {
  return (
    <ChakraProvider theme={theme}>
      <DappProvider
        environment={environment}
        customNetworkConfig={{ name: 'customConfig', apiTimeout: 6000 }}
        completedTransactionsDelay={200}
      >
        <Grid backgroundImage={
          'url(https://x-launcher.com/wp-content/uploads/2022/03/roadmap4-scaled.jpg)'
        }
        backgroundSize={'cover'}
        backgroundPosition={'center'}
        backgroundRepeat={'repeat'}>
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
