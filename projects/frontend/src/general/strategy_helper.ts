import type { Strategy } from 'src/api/Strategy';
import type { StrategyType } from 'src/api/StrategyType';
import { strategyTypeList } from './lists';

export const strategyFromType = (strategyType: StrategyType): Strategy => {
  switch (strategyType) {
    case 'winner':
      return { kind: 'winner', entry: '' };
    case 'goals':
      return {
        kind: 'goals',
        entry: {
          country_a_goals: 0,
          country_b_goals: 0,
        },
      };
    case 'cup_winner':
      return { kind: 'cup_winner', entry: '' };
    case 'game_to_penalty':
      return { kind: 'game_to_penalty', entry: false };
    case 'first_red_card':
      return { kind: 'first_red_card', entry: '' };
    case 'first_yellow_card':
      return { kind: 'first_yellow_card', entry: '' };
    case 'penalty_goals':
      return {
        kind: 'penalty_goals',
        entry: { country_a_goals: 0, country_b_goals: 0 },
      };
    case 'group_ranking':
      return { kind: 'group_ranking', entry: [] };
    case 'round_32_qualifiers':
      return { kind: 'round_32_qualifiers', entry: [] };
    case 'round_16_qualifiers':
      return { kind: 'round_16_qualifiers', entry: [] };
    case 'round_8_qualifiers':
      return { kind: 'round_8_qualifiers', entry: [] };
    case 'round_4_qualifiers':
      return { kind: 'round_4_qualifiers', entry: [] };
    case 'third_qualifiers':
      return { kind: 'third_place_qualifiers', entry: [] };
    case 'final':
      return { kind: 'final', entry: [] };
  }
};

export const validateStrategy = (strategy: Strategy): boolean => {
  switch (strategy.kind) {
    case 'winner':
      if (typeof strategy.entry !== 'string' || strategy.entry.trim() === '') {
        return false;
      }
      return true;
    case 'cup_winner':
      if (typeof strategy.entry !== 'string' || strategy.entry.trim() === '') {
        return false;
      }
      return true;
    case 'first_red_card':
      if (typeof strategy.entry !== 'string' || strategy.entry.trim() === '') {
        return false;
      }
      return true;
    case 'first_yellow_card':
      if (typeof strategy.entry !== 'string' || strategy.entry.trim() === '') {
        return false;
      }
      return true;
    case 'game_to_penalty':
      if (typeof strategy.entry !== 'boolean') {
        return false;
      }
      return true;
    case 'goals': {
      if (typeof strategy.entry !== 'object') {
        return false;
      }
      const { country_a_goals, country_b_goals } = strategy.entry;
      if (typeof country_a_goals !== 'number' || typeof country_b_goals !== 'number') {
        return false;
      }
      return true;
    }
    case 'penalty_goals': {
      if (typeof strategy.entry !== 'object') {
        return false;
      }
      const { country_a_goals, country_b_goals } = strategy.entry;
      if (typeof country_a_goals !== 'number' || typeof country_b_goals !== 'number') {
        return false;
      }
      return true;
    }
    case 'group_ranking':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 4
      ) {
        return false;
      }
      return true;
    case 'round_32_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 32
      ) {
        return false;
      }
      return true;
    case 'round_16_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 16
      ) {
        return false;
      }
      return true;
    case 'round_8_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 8
      ) {
        return false;
      }
      return true;
    case 'round_4_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 4
      ) {
        return false;
      }
      return true;
    case 'third_place_qualifiers':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 2
      ) {
        return false;
      }
      return true;
    case 'final':
      if (
        !Array.isArray(strategy.entry) ||
        strategy.entry.some((p) => typeof p !== 'string' || p.trim() === '') ||
        strategy.entry.length !== 2
      ) {
        return false;
      }
      return true;
  }
};

const list: Array<[StrategyType, Strategy]> = strategyTypeList.map((entry) => [entry, strategyFromType(entry)]);
export const strategiesToKeyValue = Object.fromEntries(list)
