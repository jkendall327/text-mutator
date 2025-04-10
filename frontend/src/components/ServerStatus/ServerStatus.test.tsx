import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ServerStatus from './ServerStatus';
import useHealthcheck from '../../hooks/useHealthcheck';
import { UseQueryResult } from '@tanstack/react-query';

vi.mock('../../hooks/useHealthcheck');
const mockedUseHealthcheck = vi.mocked(useHealthcheck);

describe('ServerStatus Component', () => {
  beforeEach(() => {
    mockedUseHealthcheck.mockClear();
  });

  it('should display loading state initially', () => {
    mockedUseHealthcheck.mockReturnValue({
      isFetching: true,
    } as UseQueryResult<string, Error>);

    const cut = render(<ServerStatus />);

    expect(screen.getByText(/Backend: Checking.../i)).toBeInTheDocument();
    const indicator = cut.container.querySelector('#loading-indicator');
    expect(indicator).toHaveClass('indicator-alive');
  });

  it('should display healthy status when data is fetched', () => {
    const healthyMessage = "Healthy";

    mockedUseHealthcheck.mockReturnValue({
      data: healthyMessage,
    } as UseQueryResult<string, Error>);

    const cut = render(<ServerStatus />);

    expect(screen.getByText(`Backend: ${healthyMessage}`)).toBeInTheDocument();
    const indicator = cut.container.querySelector('#loading-indicator');
    expect(indicator).toHaveClass('indicator-alive');
  });

  it('should display error status when an error occurs', () => {
    const errorMessage = "Network Error";

    mockedUseHealthcheck.mockReturnValue({
      error: new Error(errorMessage),
    } as UseQueryResult<string, Error>);

    const cut = render(<ServerStatus />);

    expect(screen.getByText(`Backend: Error: ${errorMessage}`)).toBeInTheDocument();
    const indicator = cut.container.querySelector('#loading-indicator');
    expect(indicator).toHaveClass('indicator-dead');
  });
});