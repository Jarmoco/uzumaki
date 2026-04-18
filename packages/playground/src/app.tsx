import { useState } from 'react';
import { C } from './theme';
import { Tab } from './types';
import { Sidebar } from './sidebar';
import { DashboardPage } from './pages/dashboardPage';
import { InputsPage } from './pages/inputsPage';
import { LayoutPage } from './pages/layoutPage';
import { StressPage } from './pages/stressPage';
import { EventsPage } from './pages/eventsPage';
import { IssuesPage } from './pages/issuesPage';

export function App() {
  const [activeTab, setActiveTab] = useState<Tab>('dashboard');

  const page = {
    dashboard: <DashboardPage />,
    inputs: <InputsPage />,
    layout: <LayoutPage />,
    stress: <StressPage />,
    events: <EventsPage />,
    issues: <IssuesPage />,
  }[activeTab];

  return (
    <view display="flex" flexDir="row" w="full" h="full" bg={C.bg}>
      <Sidebar w="16%" active={activeTab} setActive={setActiveTab} />
      <view w="84%" h="full" display="flex" flexDir="col" bg={C.bg}>
        {page}
      </view>
    </view>
  );
}
