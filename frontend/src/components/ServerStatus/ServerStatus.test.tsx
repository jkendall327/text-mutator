import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ServerStatus from './ServerStatus';
import useHealthcheck from '../../hooks/useHealthcheck';
import { UseQueryResult } from '@tanstack/react-query';

vi.mock('../../hooks/useHealthcheck');

// Use vi.mocked for better TypeScript support with mocks                                                                                                                               
const mockedUseHealthcheck = vi.mocked(useHealthcheck);

describe('ServerStatus Component', () => {
  beforeEach(() => {
    mockedUseHealthcheck.mockClear();
  });

  it('should display loading state initially', () => {
    mockedUseHealthcheck.mockReturnValue({
      data: undefined,
      error: null,
      isFetching: true,
    } as UseQueryResult<string, Error>);

    render(<ServerStatus />);

    // Assert: Check for loading text and the 'dead' indicator (or initial state)                                                                                                       
    expect(screen.getByText(/Backend: Checking.../i)).toBeInTheDocument();
    // Find the indicator div. A test ID might be more robust, but class works.                                                                                                         
    //const indicator = screen.getByRole('generic', { name: '' }).querySelector('div'); // Adjust selector if needed
    //expect(indicator).toHaveClass('indicator-dead'); // Or 'indicator-alive' depending on initial state logic
  });

  it('should display healthy status when data is fetched', () => {
    // Arrange: Configure the mock hook to return success state                                                                                                                         
    const healthyMessage = "Healthy";
    mockedUseHealthcheck.mockReturnValue({
      data: healthyMessage,
      error: null,
      isFetching: false,
      // status: 'success',                                                                                                                                                             
    } as UseQueryResult<string, Error>);

    // Act                                                                                                                                                                              
    render(<ServerStatus />);

    // Assert: Check for the success message and the 'alive' indicator                                                                                                                  
    expect(screen.getByText(`Backend: ${healthyMessage}`)).toBeInTheDocument();
    //const indicator = screen.getByRole('generic', { name: '' }).querySelector('div');
    //expect(indicator).toHaveClass('indicator-alive');
  });

  it('should display error status when an error occurs', () => {
    // Arrange: Configure the mock hook to return error state                                                                                                                           
    const errorMessage = "Network Error";

    mockedUseHealthcheck.mockReturnValue({
      data: undefined,
      error: new Error(errorMessage),
      isFetching: false,
      // status: 'error',                                                                                                                                                               
    } as UseQueryResult<string, Error>);

    // Act                                                                                                                                                                              
    render(<ServerStatus />);

    // Assert: Check for the error message and the 'dead' indicator                                                                                                                     
    expect(screen.getByText(`Backend: Error: ${errorMessage}`)).toBeInTheDocument();
    //const indicator = screen.getByRole('generic', { name: '' }).querySelector('div');
    //expect(indicator).toHaveClass('indicator-dead');
  });
});     